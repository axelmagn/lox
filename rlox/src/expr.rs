use ordered_float::OrderedFloat;

use crate::token::{Token, TokenLiteral};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: TokenLiteral,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
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
    pub fn new_assign(name: Token, value: Expr) -> Self {
        Self::Assign {
            name,
            value: Box::new(value),
        }
    }
    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Self::Call {
            callee: Box::new(callee),
            paren,
            arguments,
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

    pub fn new_literal_num(value: OrderedFloat<f64>) -> Self {
        Self::Literal {
            value: TokenLiteral::Number(value),
        }
    }

    pub fn new_literal_bool(value: bool) -> Self {
        Self::Literal {
            value: TokenLiteral::Bool(value),
        }
    }

    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
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
            Self::Assign { name, value } => visitor.visit_assign(name, value),
            Self::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Self::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(callee, paren, arguments),
            Self::Grouping { expression } => visitor.visit_grouping(expression),
            Self::Literal { value } => visitor.visit_literal(value),
            Self::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
            Self::Unary { operator, right } => visitor.visit_unary(operator, right),
            Self::Variable { name } => visitor.visit_variable(name),
        }
    }
}
pub trait ExprVisitor {
    type Output;

    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Self::Output;
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> Self::Output;
    fn visit_grouping(&mut self, expression: &Expr) -> Self::Output;
    fn visit_literal(&mut self, value: &TokenLiteral) -> Self::Output;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_variable(&mut self, name: &Token) -> Self::Output;
}
