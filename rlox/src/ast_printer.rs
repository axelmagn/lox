use crate::expr::{Expr, ExprVisitor};
use crate::token::{Token, TokenLiteral};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept_visitor(self)
    }
}

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output {
        self.parenthesize(&operator.lexeme, &vec![left, right])
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Self::Output {
        self.parenthesize("group", &vec![expression])
    }

    fn visit_literal(&mut self, value: &TokenLiteral) -> Self::Output {
        if let &TokenLiteral::Nil = value {
            return "nil".into();
        }
        value.to_string()
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output {
        self.parenthesize(&operator.lexeme, &vec![right])
    }

    fn visit_variable(&mut self, name: &Token) -> Self::Output {
        return name.lexeme.clone();
    }
}

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept_visitor(self));
        }
        builder.push(')');
        builder
    }
}
