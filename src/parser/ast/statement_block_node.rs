use crate::parser::ast::visitor::Visitor;
use crate::parser::ast::statement_node::StatementNode;

#[allow(dead_code)]
pub struct StatementBlockNode<'a> {
    pub statements: &'a Vec<&'a StatementNode>,
}

impl<'a> Visitor for StatementBlockNode<'a> {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, visitor: &Visitor) {
        for statement in self.statements {
            statement.accept(visitor);
        }
    }
}