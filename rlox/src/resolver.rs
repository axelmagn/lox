use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    expr::{Expr, ExprVisitor},
    interpreter::Interpreter,
    lox::Lox,
    stmt::{Stmt, StmtVisitor},
    token::{Token, TokenLiteral},
};

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            interpreter: interpreter.clone(),
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve_stmt_opts(&mut self, statements: &[Option<Stmt>]) {
        for stmt in statements {
            match stmt {
                Some(s) => self.resolve_stmt(s),
                _ => panic!("statement is None"),
            };
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept_visitor(self);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept_visitor(self);
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                let depth = self.scopes.len() - 1 - i;
                self.interpreter.borrow_mut().resolve(expr, depth);
                return;
            }
        }
    }

    fn resolve_function(&mut self, fparams: &[Token], fbody: &[Option<Stmt>], ftype: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = ftype;

        self.begin_scope();
        for param in fparams {
            self.declare(param);
            self.define(param);
        }
        self.resolve_stmt_opts(fbody);
        self.end_scope();

        self.current_function = enclosing_function;
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().unwrap();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(&name.lexeme) {
            Lox::error_on_token(name, "Already a variable with this name in this scope.");
        }
        scope.insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }
}

impl StmtVisitor for Resolver {
    type Output = ();

    fn visit_block(&mut self, statements: &Vec<Option<Stmt>>) -> Self::Output {
        self.begin_scope();
        self.resolve_stmt_opts(&statements);
        self.end_scope();
    }

    fn visit_class(&mut self, name: &Token, _methods: &Vec<Stmt>) -> Self::Output {
        self.declare(name);
        self.define(name);
    }

    fn visit_expression(&mut self, expression: &Expr) -> Self::Output {
        self.resolve_expr(expression);
    }

    fn visit_function(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Option<Stmt>],
    ) -> Self::Output {
        self.declare(name);
        self.define(name);
        self.resolve_function(params, body, FunctionType::Function);
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Self::Output {
        self.resolve_expr(condition);
        self.resolve_stmt(then_branch);
        if let Some(else_branch_inner) = else_branch {
            self.resolve_stmt(else_branch_inner);
        }
    }

    fn visit_print(&mut self, expression: &Expr) -> Self::Output {
        self.resolve_expr(expression);
    }

    fn visit_return(&mut self, keyword: &Token, value: &Expr) -> Self::Output {
        if self.current_function == FunctionType::None {
            Lox::error_on_token(keyword, "Can't return from top-level code.");
        }
        self.resolve_expr(value);
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::Output {
        self.declare(name);
        if let Some(initializer_expr) = initializer {
            self.resolve_expr(initializer_expr);
        }
        self.define(name);
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output {
        self.resolve_expr(condition);
        self.resolve_stmt(body);
    }
}

impl ExprVisitor for Resolver {
    type Output = ();

    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Self::Output {
        let expr = Expr::new_assign(name.clone(), value.clone());
        self.resolve_expr(value);
        self.resolve_local(&expr, name);
    }

    fn visit_binary(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Self::Output {
        self.resolve_expr(left);
        self.resolve_expr(right);
    }

    fn visit_call(&mut self, callee: &Expr, _paren: &Token, arguments: &[Expr]) -> Self::Output {
        self.resolve_expr(callee);
        for argument in arguments {
            self.resolve_expr(argument);
        }
    }

    fn visit_get(&mut self, object: &Expr, _name: &Token) -> Self::Output {
        self.resolve_expr(object);
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Self::Output {
        self.resolve_expr(expression);
    }

    fn visit_literal(&mut self, _value: &TokenLiteral) -> Self::Output {}

    fn visit_logical(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Self::Output {
        self.resolve_expr(left);
        self.resolve_expr(right);
    }

    fn visit_set(&mut self, object: &Expr, _name: &Token, value: &Expr) -> Self::Output {
        self.resolve_expr(value);
        self.resolve_expr(object);
    }

    fn visit_unary(&mut self, _operator: &Token, right: &Expr) -> Self::Output {
        self.resolve_expr(right);
    }

    fn visit_variable(&mut self, name: &Token) -> Self::Output {
        if !self.scopes.is_empty() && self.scopes.last().unwrap().get(&name.lexeme) == Some(&false)
        {
            Lox::error_on_token(name, "Can't read local variable in its own initializer.");
        }
        self.resolve_local(&Expr::new_variable(name.clone()), name);
    }
}
