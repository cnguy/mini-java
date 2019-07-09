// Visitor allows clients to implement the visitor pattern.
pub trait Visitor {
    fn accept(&self, visitor: &Visitor);
    fn visit(&self, visitor: &Visitor);
}
