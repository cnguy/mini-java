use crate::parser::ast::visitor::Visitor;
use crate::parser::ast::expression_node::ExpressionNode;

#[allow(dead_code)]
pub struct IntegerNode {
    pub value: i64,
}

impl ExpressionNode for IntegerNode {}

impl Visitor for IntegerNode {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, _visitor: &Visitor) {}
}
