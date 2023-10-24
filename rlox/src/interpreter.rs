use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    errors::RuntimeError,
    expr::{Expr, ExprVisitor},
    lox::Lox,
    stmt::{Stmt, StmtVisitor},
    token::{Token, TokenLiteral, TokenType},
    value::Value,
};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            let res = self.execute(stmt);
            match res {
                Err(e) => Lox::runtime_error(e),
                _ => {}
            };
        }
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        statement.accept_visitor(self)
    }

    fn execute_block(
        &mut self,
        statements: &Vec<Option<Stmt>>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        let previous = self.environment.clone();
        self.environment = environment;
        let mut res = Ok(());
        for statement_opt in statements {
            res = self.execute(statement_opt.as_ref().unwrap());
            if res.is_err() {
                break;
            }
        }
        self.environment = previous;
        res
    }

    fn evaluate(&mut self, expr: &Expr) -> <Self as ExprVisitor>::Output {
        expr.accept_visitor(self)
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(v) => *v,
            _ => true,
        }
    }

    fn check_number_operand(
        &self,
        operator: &crate::token::Token,
        operand: &Value,
    ) -> Result<f64, RuntimeError> {
        match operand {
            Value::Number(n) => Ok(*n),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operand must be a number.".into(),
            )),
        }
    }
    fn check_number_operands(
        &self,
        operator: &crate::token::Token,
        left: &Value,
        right: &Value,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok((*l, *r)),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operands must be numbers.".into(),
            )),
        }
    }

    fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Nil => "nil".into(),
            Value::Number(v) => {
                let mut text = v.to_string();
                if text.ends_with(".0") {
                    text = text[..text.len() - 2].into();
                }
                text
            }
            Value::String(v) => v.clone(),
            Value::Bool(v) => v.to_string(),
        }
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), RuntimeError>;

    fn visit_block(&mut self, statements: &Vec<Option<Stmt>>) -> Self::Output {
        let env = Environment::with_enclosing(self.environment.clone());
        self.execute_block(statements, Rc::new(RefCell::new(env)))
    }

    fn visit_expression(&mut self, expression: &Expr) -> Self::Output {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Self::Output {
        let cond_res = self.evaluate(condition)?;
        if self.is_truthy(&cond_res) {
            self.execute(then_branch)?;
        } else if else_branch.is_some() {
            self.execute(else_branch.as_ref().unwrap())?;
        }
        Ok(())
    }

    fn visit_print(&mut self, expression: &Expr) -> Self::Output {
        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::Output {
        let mut value = Value::Nil;
        match initializer {
            Some(expr) => {
                value = self.evaluate(expr)?;
            }
            _ => {}
        };
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output {
        while {
            let cond_res = self.evaluate(condition)?;
            self.is_truthy(&cond_res)
        } {
            self.execute(body)?;
        }
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value, RuntimeError>;

    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Self::Output {
        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.ttype {
            TokenType::BangEqual => Ok(Value::from(left != right)),
            TokenType::EqualEqual => Ok(Value::from(left == right)),
            TokenType::Greater => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                Ok(Value::from(left > right))
            }
            TokenType::GreaterEqual => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                Ok(Value::from(left >= right))
            }
            TokenType::Less => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                Ok(Value::from(left < right))
            }
            TokenType::LessEqual => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                Ok(Value::from(left <= right))
            }
            TokenType::Minus => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                Ok(Value::from(left - right))
            }
            TokenType::Plus => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::from(left + right)),
                (Value::String(left), Value::String(right)) => Ok(Value::from(left + &right)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be two numbers or two strings.".into(),
                )),
            },
            TokenType::Slash => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                Ok(Value::from(left / right))
            }
            TokenType::Star => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                Ok(Value::from(left * right))
            }
            _ => panic!("unreachable"),
        }
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Self::Output {
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, literal: &TokenLiteral) -> Self::Output {
        Ok(Value::from(literal.clone()))
    }

    fn visit_unary(
        &mut self,
        operator: &crate::token::Token,
        right: &crate::expr::Expr,
    ) -> Self::Output {
        let right = self.evaluate(right)?;
        match operator.ttype {
            TokenType::Bang => Ok(Value::Bool(!self.is_truthy(&right))),
            TokenType::Minus => {
                let n = self.check_number_operand(operator, &right)?;
                Ok(Value::Number(-n))
            }
            _ => panic!("unreachable"),
        }
    }

    fn visit_variable(&mut self, name: &crate::token::Token) -> Self::Output {
        self.environment.borrow().get(name)
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &crate::token::Token,
        right: &Expr,
    ) -> Self::Output {
        let left = self.evaluate(left)?;

        // check if we can short circuit
        if operator.ttype == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        return self.evaluate(right);
    }
}
