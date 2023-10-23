use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Block {
        statements: Vec<Option<Stmt>>,
    },
    Expression {
        expression: Box<Expr>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Option<Expr>>,
    },
}

impl Stmt {
    pub fn new_block(statements: Vec<Option<Stmt>>) -> Self {
        Self::Block { statements }
    }

    pub fn new_expression(expression: Expr) -> Self {
        Self::Expression {
            expression: Box::new(expression),
        }
    }

    pub fn new_print(expression: Expr) -> Self {
        Self::Print {
            expression: Box::new(expression),
        }
    }

    pub fn new_var(name: Token, initializer: Option<Expr>) -> Self {
        Self::Var {
            name,
            initializer: Box::new(initializer),
        }
    }

    pub fn accept_visitor<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Self::Block { statements } => visitor.visit_block(statements),
            Self::Expression { expression } => visitor.visit_expression(expression),
            Self::Print { expression } => visitor.visit_print(expression),
            Self::Var { name, initializer } => visitor.visit_var(name, initializer),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_block(&mut self, statements: &Vec<Option<Stmt>>) -> Self::Output;
    fn visit_expression(&mut self, expression: &Expr) -> Self::Output;
    fn visit_print(&mut self, expression: &Expr) -> Self::Output;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::Output;
}
