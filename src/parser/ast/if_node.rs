use crate::parser::ast::expression_node::ExpressionNode;
use crate::parser::ast::statement_block_node::StatementBlockNode;
use crate::parser::ast::statement_node::StatementNode;
use crate::parser::ast::visitor::Visitor;

#[allow(dead_code)]
pub struct IfNode<'a> {
    pub condition: &'a ExpressionNode,
    pub statement_block: &'a StatementBlockNode<'a>,
}

impl<'a> StatementNode for IfNode<'a> {}

impl<'a> Visitor for IfNode<'a> {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit(self);
    }

    fn visit(&self, visitor: &Visitor) {
        self.condition.accept(visitor);
        self.statement_block.accept(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::ast::integer_node::IntegerNode;
    use crate::parser::ast::unary_expr_node::UnaryExprNode;
    use crate::parser::ast::if_node::IfNode;

    #[test]
    fn test_compiles_1() {
        let integer = IntegerNode { value: 4 };
        let unary_expr = UnaryExprNode { expr: &integer };
        let statement_block = StatementBlockNode {
            statements: &vec![],
        };
        let _if_node = IfNode {
            condition: &unary_expr,
            statement_block: &statement_block,
        };
    }
}
