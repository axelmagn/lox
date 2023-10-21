use crate::token::{Token, TokenLiteral};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: TokenLiteral,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub trait ExprVisitor {
    type Result;

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Result;
    fn visit_grouping(&mut self, expression: &Expr) -> Self::Result;
    fn visit_literal(&mut self, value: &TokenLiteral) -> Self::Result;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Result;
}

pub trait ExprVisitorData {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Result;
}

impl ExprVisitorData for Expr {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Result {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Self::Grouping { expression } => visitor.visit_grouping(expression),
            Self::Literal { value } => visitor.visit_literal(value),
            Self::Unary { operator, right } => visitor.visit_unary(operator, right),
        }
    }
}
