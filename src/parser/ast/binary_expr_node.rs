use crate::parser::ast::visitor::Visitor;
use crate::parser::ast::expression_node::ExpressionNode;

#[allow(dead_code)]
struct BinaryExprNode<'a> {
    pub left: &'a ExpressionNode,
    pub right: &'a ExpressionNode,
}

impl<'a> ExpressionNode for BinaryExprNode<'a> {}

impl<'a> Visitor for BinaryExprNode<'a> {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, visitor: &Visitor) {
        self.left.accept(visitor);
        self.right.accept(visitor);
    }
}
