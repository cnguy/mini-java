use crate::parser::ast::expression_node::ExpressionNode;
use crate::parser::ast::visitor::Visitor;

#[allow(dead_code)]
pub struct UnaryExprNode<'a> {
    pub expr: &'a ExpressionNode,
}

impl<'a> ExpressionNode for UnaryExprNode<'a> {}

impl<'a> Visitor for UnaryExprNode<'a> {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, visitor: &Visitor) {
        self.expr.accept(visitor);
    }
}

