use plc::ast::{AstStatement, Operator};

use crate::model::{
    fbd::{Node, NodeIndex},
    variables::{BlockVariable, FunctionBlockVariable},
};

use super::ParseSession;

impl BlockVariable {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> Option<AstStatement> {
        let Some(ref_id) = &self.ref_local_id else {
            // param not provided/passed
            return None;
        };

        match index.get(ref_id) {
            Some(Node::Block(block)) => Some(block.transform(session, index)),
            Some(Node::FunctionBlockVariable(var)) => Some(var.transform(session)),
            Some(Node::Control(_)) => todo!(),
            Some(Node::Connector(_)) => todo!(),
            None => unreachable!(),
        }
    }
}

impl FunctionBlockVariable {
    pub(crate) fn transform(&self, session: &ParseSession) -> AstStatement {
        let stmt = if self.negated {
            let ident = session.parse_expression(&self.expression);
            let location = ident.get_location();
            AstStatement::UnaryExpression {
                operator: Operator::Not,
                value: Box::new(ident),
                location,
                id: session.next_id(),
            }
        } else {
            session.parse_expression(&self.expression)
        };

        stmt
    }
}
