use crate::parser::ast::visitor::Visitor;

#[allow(dead_code)]
struct IntegerNode {
    pub value: i64,
}

impl Visitor for IntegerNode {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, _visitor: &Visitor) {}
}
