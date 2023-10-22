use crate::{
    errors::RuntimeError,
    expr::{Expr, ExprVisitor, ExprVisitorData},
    lox::Lox,
    token::TokenType,
    value::Value,
};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&mut self, expression: &Expr) {
        match self.evaluate(expression) {
            Ok(v) => println!("{}", self.stringify(&v)),
            Err(e) => Lox::runtime_error(e),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> <Self as ExprVisitor>::Output {
        expr.accept(self)
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

impl ExprVisitor for Interpreter {
    type Output = Result<Value, RuntimeError>;

    fn visit_binary(
        &mut self,
        left: &crate::expr::Expr,
        operator: &crate::token::Token,
        right: &crate::expr::Expr,
    ) -> Self::Output {
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

    fn visit_grouping(&mut self, expression: &crate::expr::Expr) -> Self::Output {
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, literal: &crate::token::TokenLiteral) -> Self::Output {
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
}
