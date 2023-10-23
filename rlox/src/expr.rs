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
    Variable {
        name: Token,
    },
}

impl Expr {
    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_grouping(expression: Expr) -> Self {
        Self::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn new_literal(literal: TokenLiteral) -> Self {
        Self::Literal { value: literal }
    }

    pub fn new_literal_nil() -> Self {
        Self::Literal {
            value: TokenLiteral::Nil,
        }
    }

    pub fn new_literal_str(value: String) -> Self {
        Self::Literal {
            value: TokenLiteral::String(value),
        }
    }

    pub fn new_literal_num(value: f64) -> Self {
        Self::Literal {
            value: TokenLiteral::Number(value),
        }
    }

    pub fn new_literal_bool(value: bool) -> Self {
        Self::Literal {
            value: TokenLiteral::Bool(value),
        }
    }

    pub fn new_unary(operator: Token, right: Expr) -> Self {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_variable(name: Token) -> Self {
        Self::Variable { name }
    }

    pub fn accept_visitor<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Self::Grouping { expression } => visitor.visit_grouping(expression),
            Self::Literal { value } => visitor.visit_literal(value),
            Self::Unary { operator, right } => visitor.visit_unary(operator, right),
            Self::Variable { name } => visitor.visit_variable(name),
        }
    }
}
pub trait ExprVisitor {
    type Output;

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_grouping(&mut self, expression: &Expr) -> Self::Output;
    fn visit_literal(&mut self, value: &TokenLiteral) -> Self::Output;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_variable(&mut self, name: &Token) -> Self::Output;
}
