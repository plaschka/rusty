// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables
use crate::{
    ast::{LinkageType, SourceRange},
    codegen::{debug::Debug, llvm_index::LlvmTypedIndex, llvm_typesystem::cast_if_needed},
    diagnostics::{Diagnostic, ErrNo},
    index::{get_initializer_name, Index, PouIndexEntry, VariableIndexEntry},
    resolver::{AnnotationMap, AstAnnotations, Dependency},
};
use indexmap::IndexSet;
use inkwell::{module::Module, values::GlobalValue};

use super::{
    data_type_generator::get_default_for,
    expression_generator::ExpressionCodeGenerator,
    llvm::{GlobalValueExt, Llvm},
};
use crate::codegen::debug::DebugBuilderEnum;

pub fn generate_global_variables<'ctx, 'b>(
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    debug: &'b mut DebugBuilderEnum<'ctx>,
    dependencies: &IndexSet<Dependency>,
    global_index: &'b Index,
    annotations: &'b AstAnnotations,
    types_index: &'b LlvmTypedIndex<'ctx>,
    location: &'b str,
) -> Result<LlvmTypedIndex<'ctx>, Diagnostic> {
    let mut index = LlvmTypedIndex::default();

    let mut globals = vec![];
    dependencies.iter().for_each(|dep| {
        if let Some(dep) = match dep {
            Dependency::Datatype(name) => {
                //Attempt to find a pou with that name
                if let Some(PouIndexEntry::Program { instance_variable, .. }) =
                    global_index.find_pou(name).as_ref()
                {
                    Some((name.as_str(), instance_variable))
                } else {
                    None
                }
            }
            Dependency::Variable(name) => {
                global_index.find_fully_qualified_variable(name).map(|it| (name.as_str(), it))
            }
            Dependency::Call(_) => None,
        } {
            globals.push(dep);
        }

        if let Some(init) = global_index.find_global_initializer(&get_initializer_name(dep.get_name())) {
            globals.push((init.get_name(), init));
        }
    });

    for (name, variable) in globals {
        let linkage =
            if !variable.is_in_unit(location) { LinkageType::External } else { variable.get_linkage() };
        let global_variable =
            generate_global_variable(module, llvm, global_index, annotations, types_index, variable, linkage)
                .map_err(|err| match err.get_type() {
                    ErrNo::codegen__missing_function | ErrNo::reference__unresolved => {
                        Diagnostic::cannot_generate_initializer(name, SourceRange::undefined())
                    }
                    _ => err,
                })?;
        index.associate_global(name, global_variable)?;
        //generate debug info
        debug.create_global_variable(
            variable.get_qualified_name(),
            &variable.data_type_name,
            global_variable,
            &variable.source_location,
        );
    }

    Ok(index)
}

/// convenience function to generates a global variable for the given variable
///
/// - `module` the module to generate the variable into
/// - `llvm` the struct used to generate IR-code
/// - `index` the global symbol table, the global variable will be registerd as a new symbol
/// - `global_variable` the variable to generate
fn generate_global_variable<'ctx, 'b>(
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    global_index: &'b Index,
    annotations: &'b AstAnnotations,
    index: &'b LlvmTypedIndex<'ctx>,
    global_variable: &VariableIndexEntry,
    linkage: LinkageType,
) -> Result<GlobalValue<'ctx>, Diagnostic> {
    let type_name = global_variable.get_type_name();
    let variable_type = index.get_associated_type(type_name)?;

    let mut global_ir_variable =
        llvm.create_global_variable(module, global_variable.get_name(), variable_type);
    if linkage == LinkageType::External {
        global_ir_variable = global_ir_variable.make_external();
    } else {
        let initial_value = if let Some(initializer) =
            global_index.get_const_expressions().maybe_get_constant_statement(&global_variable.initial_value)
        {
            let expr_generator =
                ExpressionCodeGenerator::new_context_free(llvm, global_index, annotations, index);

            //see if this value was compile-time evaluated ...
            if let Some(value) = index.find_constant_value(global_variable.get_qualified_name()) {
                Some(value)
            } else {
                let value = expr_generator.generate_expression(initializer)?;
                let target_type = global_index.get_effective_type_or_void_by_name(type_name);
                let value_type = annotations.get_type_or_void(initializer, global_index);
                Some(cast_if_needed!(expr_generator, target_type, value_type, value, None))
            }
        } else {
            None
        };

        let initial_value = initial_value
            // 2nd try: find an associated default value for the declared type
            .or_else(|| index.find_associated_initial_value(type_name))
            // 3rd try: get the compiler's default for the given type (zero-initializer)
            .or_else(|| index.find_associated_type(type_name).map(get_default_for));
        global_ir_variable.set_initial_value(initial_value, variable_type);
        if global_variable.is_constant() {
            global_ir_variable = global_ir_variable.make_constant();
            if initial_value.is_none() {
                return Err(Diagnostic::codegen_error(
                    "Cannot generate uninitialized constant",
                    global_variable.source_location.source_range.clone(),
                ));
            }
        }
    }

    Ok(global_ir_variable)
}
