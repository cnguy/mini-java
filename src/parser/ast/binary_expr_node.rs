use crate::parser::ast::visitor::Visitor;

#[allow(dead_code)]
struct BinaryExprNode<'a> {
    pub left: &'a Visitor,
    pub right: &'a Visitor,
}

impl<'a> Visitor for BinaryExprNode<'a> {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, visitor: &Visitor) {
        self.left.accept(visitor);
        self.right.accept(visitor);
    }
}
