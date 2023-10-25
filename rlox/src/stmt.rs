use std::rc::Rc;

use crate::{expr::Expr, token::Token};

#[derive(Clone)]
pub enum Stmt {
    Block {
        statements: Vec<Option<Stmt>>,
    },
    Expression {
        expression: Rc<Expr>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Option<Stmt>>,
    },
    If {
        condition: Rc<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    Print {
        expression: Rc<Expr>,
    },
    Return {
        keyword: Token,
        value: Rc<Expr>,
    },
    Var {
        name: Token,
        initializer: Rc<Option<Expr>>,
    },
    While {
        condition: Rc<Expr>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn new_block(statements: Vec<Option<Stmt>>) -> Self {
        Self::Block { statements }
    }

    pub fn new_expression(expression: Expr) -> Self {
        Self::Expression {
            expression: Rc::new(expression),
        }
    }

    pub fn new_function(name: Token, params: Vec<Token>, body: Vec<Option<Stmt>>) -> Self {
        Self::Function { name, params, body }
    }

    pub fn new_if(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Self::If {
            condition: Rc::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    pub fn new_print(expression: Expr) -> Self {
        Self::Print {
            expression: Rc::new(expression),
        }
    }

    pub fn new_return(keyword: Token, value: Expr) -> Self {
        Self::Return {
            keyword,
            value: Rc::new(value),
        }
    }

    pub fn new_var(name: Token, initializer: Option<Expr>) -> Self {
        Self::Var {
            name,
            initializer: Rc::new(initializer),
        }
    }
    pub fn new_while(condition: Expr, body: Stmt) -> Self {
        Self::While {
            condition: Rc::new(condition),
            body: Box::new(body),
        }
    }

    pub fn accept_visitor<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Self::Block { statements } => visitor.visit_block(statements),
            Self::Expression { expression } => visitor.visit_expression(expression),
            Self::Function { name, params, body } => visitor.visit_function(name, params, body),
            Self::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if(condition, then_branch, else_branch),
            Self::Print { expression } => visitor.visit_print(expression),
            Self::Return { keyword, value } => visitor.visit_return(keyword, value),
            Self::Var { name, initializer } => visitor.visit_var(name, initializer),
            Self::While { condition, body } => visitor.visit_while(condition, body),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_block(&mut self, statements: &Vec<Option<Stmt>>) -> Self::Output;
    fn visit_expression(&mut self, expression: &Expr) -> Self::Output;
    fn visit_function(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Option<Stmt>],
    ) -> Self::Output;
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Self::Output;
    fn visit_print(&mut self, expression: &Expr) -> Self::Output;
    fn visit_return(&mut self, keyword: &Token, value: &Expr) -> Self::Output;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::Output;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output;
}
