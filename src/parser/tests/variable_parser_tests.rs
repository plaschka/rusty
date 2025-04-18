use plc_ast::ast::{LinkageType, VariableBlock};

use crate::test_utils::tests::parse;

#[test]
fn empty_global_vars_can_be_parsed() {
    let src = "VAR_GLOBAL END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn global_vars_can_be_parsed() {
    let src = "VAR_GLOBAL x : INT; y : BOOL; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn external_global_vars_can_be_parsed() {
    let src = "@EXTERNAL VAR_GLOBAL x : INT; y : BOOL; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast);
    assert!(matches!(vars, VariableBlock { linkage: LinkageType::External, .. }));
}

#[test]
fn global_single_line_vars_can_be_parsed() {
    let src = "VAR_GLOBAL x, y,z : INT; f : BOOL; b, c : SINT; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "z",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "f",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
        Variable {
            name: "b",
            data_type: DataTypeReference {
                referenced_type: "SINT",
            },
        },
        Variable {
            name: "c",
            data_type: DataTypeReference {
                referenced_type: "SINT",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn two_global_vars_can_be_parsed() {
    let src = "VAR_GLOBAL a: INT; END_VAR VAR_GLOBAL x : INT; y : BOOL; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars; //global_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"[
    VariableBlock {
        variables: [
            Variable {
                name: "a",
                data_type: DataTypeReference {
                    referenced_type: "INT",
                },
            },
        ],
        variable_block_type: Global,
    },
    VariableBlock {
        variables: [
            Variable {
                name: "x",
                data_type: DataTypeReference {
                    referenced_type: "INT",
                },
            },
            Variable {
                name: "y",
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
            },
        ],
        variable_block_type: Global,
    },
]"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn global_var_with_address() {
    let src = "VAR_GLOBAL
            a AT %I* : INT;
            b AT %Q* : INT;
            c AT %M* : INT;
            aa AT %IX7 : INT;
            bb AT %QB5.5 : INT;
            cc AT %MD3.3.3 : INT;
            dd AT %GD4.3.3 : INT;
    END_VAR ";
    let (result, diag) = parse(src);

    assert_eq!(diag, vec![]);

    insta::assert_snapshot!(format!("{result:?}"));
}

#[test]
fn pou_var_with_address() {
    let src = "PROGRAM main
    VAR
            a AT %I* : INT;
            b AT %Q* : INT;
            c,d AT %M* : INT;
            aa AT %IX7 : INT;
            bb AT %QB5.5 : INT;
            cc AT %MD3.3.3 : INT;
            dd AT %GD4.3.3 : INT;
    END_VAR
    END_PROGRAM
    ";
    let (result, diag) = parse(src);

    assert_eq!(diag, vec![]);
    insta::assert_debug_snapshot!(result);
}

#[test]
fn struct_with_address() {
    let src = "TYPE t : STRUCT
            a AT %I* : INT;
            b AT %Q* : INT;
            c AT %M* : INT;
            aa AT %IX7 : INT;
            bb AT %QB5.5 : INT;
            cc AT %MD3.3.3 : INT;
            dd AT %GD4.3.3 : INT;
    END_STRUCT
    END_TYPE
    ";
    let (result, diag) = parse(src);

    assert_eq!(diag, vec![]);
    insta::assert_snapshot!(format!("{result:?}"));
}

#[test]
fn date_and_time_constants_test() {
    let src = r#"
    VAR_GLOBAL CONSTANT
        cT          : TIME;
        cT_SHORT    : TIME;
        cLT         : LTIME;
        cLT_SHORT   : LTIME;
        cD          : DATE;
        cD_SHORT    : DATE;
        cLD         : LDATE;
        cLD_SHORT   : LDATE;
        cTOD        : TIME_OF_DAY;
        cTOD_SHORT  : TOD;
        cLTOD       : LTOD;
        cLTOD_SHORT : LTOD;
        cDT         : DATE_AND_TIME;
        cDT_SHORT   : DT;
        cLDT        : LDT;
        cLDT_SHORT  : LDT;
    END_VAR"#;

    let (result, diag) = parse(src);
    let vars = &result.global_vars[0]; //globar_vars
    assert_eq!(diag, vec![]);
    insta::assert_snapshot!(format!("{vars:#?}"));
}

#[test]
fn var_config_test() {
    let src = "
    VAR_CONFIG
        instance1.foo.qux AT %IX3.1 : BOOL;
        instance2.bar.qux AT %IX5.6 : BOOL;
    END_VAR
    ";
    let (result, diag) = parse(src);

    assert!(diag.is_empty());
    insta::assert_debug_snapshot!(result, @r###"
    CompilationUnit {
        global_vars: [],
        var_config: [
            ConfigVariable {
                reference: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "qux",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "foo",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instance1",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
                address: HardwareAccess {
                    direction: Input,
                    access: Bit,
                    address: [
                        LiteralInteger {
                            value: 3,
                        },
                        LiteralInteger {
                            value: 1,
                        },
                    ],
                    location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 2,
                                column: 26,
                                offset: 42,
                            }..TextLocation {
                                line: 2,
                                column: 35,
                                offset: 51,
                            },
                        ),
                    },
                },
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 2,
                            column: 8,
                            offset: 24,
                        }..TextLocation {
                            line: 2,
                            column: 25,
                            offset: 41,
                        },
                    ),
                },
            },
            ConfigVariable {
                reference: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "qux",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "bar",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instance2",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
                address: HardwareAccess {
                    direction: Input,
                    access: Bit,
                    address: [
                        LiteralInteger {
                            value: 5,
                        },
                        LiteralInteger {
                            value: 6,
                        },
                    ],
                    location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 3,
                                column: 26,
                                offset: 86,
                            }..TextLocation {
                                line: 3,
                                column: 35,
                                offset: 95,
                            },
                        ),
                    },
                },
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 3,
                            column: 8,
                            offset: 68,
                        }..TextLocation {
                            line: 3,
                            column: 25,
                            offset: 85,
                        },
                    ),
                },
            },
        ],
        pous: [],
        implementations: [],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "###);
}

#[test]
fn var_config_location() {
    let src = r#"
    VAR_CONFIG
        main.instance.foo AT %IX3.1 : BOOL;
    END_VAR
    "#;

    let (result, _) = parse(src);

    assert_eq!("main.instance.foo", &src[result.var_config[0].location.to_range().unwrap()]);
}

#[test]
fn var_external() {
    let src = r#"
    VAR_GLOBAL 
        arr: ARRAY [0..100] OF INT; 
    END_VAR

    FUNCTION foo
    VAR_EXTERNAL 
        arr : ARRAY [0..100] OF INT;
    END_VAR
    END_FUNCTION
    "#;

    let (result, _) = parse(src);

    insta::assert_debug_snapshot!(result, @r###"
    CompilationUnit {
        global_vars: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "arr",
                        data_type: DataTypeDefinition {
                            data_type: ArrayType {
                                name: None,
                                bounds: RangeStatement {
                                    start: LiteralInteger {
                                        value: 0,
                                    },
                                    end: LiteralInteger {
                                        value: 100,
                                    },
                                },
                                referenced_type: DataTypeReference {
                                    referenced_type: "INT",
                                },
                                is_variable_length: false,
                            },
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ],
        var_config: [],
        pous: [
            POU {
                name: "foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "arr",
                                data_type: DataTypeDefinition {
                                    data_type: ArrayType {
                                        name: None,
                                        bounds: RangeStatement {
                                            start: LiteralInteger {
                                                value: 0,
                                            },
                                            end: LiteralInteger {
                                                value: 100,
                                            },
                                        },
                                        referenced_type: DataTypeReference {
                                            referenced_type: "INT",
                                        },
                                        is_variable_length: false,
                                    },
                                },
                            },
                        ],
                        variable_block_type: External,
                    },
                ],
                pou_type: Function,
                return_type: None,
                interfaces: [],
                properties: [],
            },
        ],
        implementations: [
            Implementation {
                name: "foo",
                type_name: "foo",
                linkage: Internal,
                pou_type: Function,
                statements: [],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 9,
                            column: 4,
                            offset: 155,
                        }..TextLocation {
                            line: 8,
                            column: 11,
                            offset: 150,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 13,
                            offset: 80,
                        }..TextLocation {
                            line: 5,
                            column: 16,
                            offset: 83,
                        },
                    ),
                },
                end_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 9,
                            column: 4,
                            offset: 155,
                        }..TextLocation {
                            line: 9,
                            column: 16,
                            offset: 167,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: None,
            },
        ],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "###);
}

#[test]
fn var_external_constant() {
    let src = r#"
    VAR_GLOBAL 
        arr: ARRAY [0..100] OF INT; 
    END_VAR

    FUNCTION foo
    VAR_EXTERNAL CONSTANT
        arr : ARRAY [0..100] OF INT;
    END_VAR
    END_FUNCTION
    "#;

    let (result, _) = parse(src);

    insta::assert_debug_snapshot!(result, @r###"
    CompilationUnit {
        global_vars: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "arr",
                        data_type: DataTypeDefinition {
                            data_type: ArrayType {
                                name: None,
                                bounds: RangeStatement {
                                    start: LiteralInteger {
                                        value: 0,
                                    },
                                    end: LiteralInteger {
                                        value: 100,
                                    },
                                },
                                referenced_type: DataTypeReference {
                                    referenced_type: "INT",
                                },
                                is_variable_length: false,
                            },
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ],
        var_config: [],
        pous: [
            POU {
                name: "foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "arr",
                                data_type: DataTypeDefinition {
                                    data_type: ArrayType {
                                        name: None,
                                        bounds: RangeStatement {
                                            start: LiteralInteger {
                                                value: 0,
                                            },
                                            end: LiteralInteger {
                                                value: 100,
                                            },
                                        },
                                        referenced_type: DataTypeReference {
                                            referenced_type: "INT",
                                        },
                                        is_variable_length: false,
                                    },
                                },
                            },
                        ],
                        variable_block_type: External,
                    },
                ],
                pou_type: Function,
                return_type: None,
                interfaces: [],
                properties: [],
            },
        ],
        implementations: [
            Implementation {
                name: "foo",
                type_name: "foo",
                linkage: Internal,
                pou_type: Function,
                statements: [],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 9,
                            column: 4,
                            offset: 163,
                        }..TextLocation {
                            line: 8,
                            column: 11,
                            offset: 158,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 13,
                            offset: 80,
                        }..TextLocation {
                            line: 5,
                            column: 16,
                            offset: 83,
                        },
                    ),
                },
                end_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 9,
                            column: 4,
                            offset: 163,
                        }..TextLocation {
                            line: 9,
                            column: 16,
                            offset: 175,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: None,
            },
        ],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "###);
}
