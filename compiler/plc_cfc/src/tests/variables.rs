#[cfg(test)]
mod tests {
    use crate::{
        deserializer::Parseable,
        reader::PeekableReader,
        serializer::{Block, InOutVariables, InputVariables, OutputVariables, Variable},
    };

    #[test]
    fn variable() {
        let content = Block::init("1", "bar")
            .with_input_variables(
                InputVariables::new()
                    .with_variable(Variable::init("a", false))
                    .with_variable(Variable::init("b", false)),
            )
            .with_output_variables(
                OutputVariables::new()
                    .with_variable(Variable::init("c", true))
                    .with_variable(Variable::init("d", true)),
            )
            .with_inout_variables(InOutVariables::new().close())
            .serialize();

        let mut reader = PeekableReader::new(&content);
        insta::assert_debug_snapshot!(crate::model::block::Block::visit(&mut reader));
    }
}
