use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Block {
        statements: Vec<Option<Stmt>>,
    },
    Expression {
        expression: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Option<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
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

    pub fn new_if(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Self::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
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
    pub fn new_while(condition: Expr, body: Stmt) -> Self {
        Self::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    pub fn accept_visitor<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Self::Block { statements } => visitor.visit_block(statements),
            Self::Expression { expression } => visitor.visit_expression(expression),
            Self::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if(condition, then_branch, else_branch),
            Self::Print { expression } => visitor.visit_print(expression),
            Self::Var { name, initializer } => visitor.visit_var(name, initializer),
            Self::While { condition, body } => visitor.visit_while(condition, body),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_block(&mut self, statements: &Vec<Option<Stmt>>) -> Self::Output;
    fn visit_expression(&mut self, expression: &Expr) -> Self::Output;
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Self::Output;
    fn visit_print(&mut self, expression: &Expr) -> Self::Output;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::Output;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output;
}
