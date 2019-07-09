use crate::parser::ast::visitor::Visitor;

#[allow(dead_code)]
struct UnaryExprNode<'a> {
    pub expr: &'a Visitor,
}

impl<'a> Visitor for UnaryExprNode<'a> {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, visitor: &Visitor) {
        self.expr.accept(visitor);
    }
}

