use crate::expr::{Expr, ExprVisitor, ExprVisitorData};
use crate::token::{Token, TokenLiteral};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor for AstPrinter {
    type Result = String;

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Result {
        self.parenthesize(&operator.lexeme, &vec![left, right])
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Self::Result {
        self.parenthesize("group", &vec![expression])
    }

    fn visit_literal(&mut self, value: &TokenLiteral) -> Self::Result {
        if let &TokenLiteral::Nil = value {
            return "nil".into();
        }
        value.to_string()
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Result {
        self.parenthesize(&operator.lexeme, &vec![right])
    }
}

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}
