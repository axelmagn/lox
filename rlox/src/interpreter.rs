use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ordered_float::OrderedFloat;

use crate::{
    environment::Environment,
    errors::RuntimeError,
    expr::{Expr, ExprVisitor},
    lox::Lox,
    lox_callable::LoxCallable,
    lox_class::LoxClass,
    lox_function::LoxFunction,
    lox_instance::LoxInstance,
    native_functions::CLOCK_FN,
    stmt::{Stmt, StmtVisitor},
    token::{Token, TokenLiteral, TokenType},
    value::Value,
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut environment = Environment::new();
        environment.define("clock".into(), Value::NativeFn(&CLOCK_FN));
        let globals = Rc::new(RefCell::new(environment));
        Self {
            environment: globals.clone(),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            let res = self.execute(stmt);
            match res {
                Err(e) => {
                    Lox::runtime_error(e);
                    return;
                }
                _ => {}
            };
        }
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    fn look_up_variable(&self, name: &Token, expr: &Expr) -> Result<Value, RuntimeError> {
        let distance = self.locals.get(expr);
        if let Some(distance) = distance {
            Ok(self
                .environment
                .borrow()
                .get_at(*distance, &name.lexeme)
                .unwrap())
        } else {
            self.globals.borrow().get(name)
        }
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        statement.accept_visitor(self)
    }

    pub fn execute_block(
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
    ) -> Result<OrderedFloat<f64>, RuntimeError> {
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
    ) -> Result<(OrderedFloat<f64>, OrderedFloat<f64>), RuntimeError> {
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
            Value::NativeFn(v) => v.string_repr(),
            Value::LoxFn(v) => v.string_repr(),
            Value::LoxClass(v) => v.to_string(),
            Value::LoxInstance(v) => v.borrow().to_string(),
        }
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), RuntimeError>;

    fn visit_block(&mut self, statements: &Vec<Option<Stmt>>) -> Self::Output {
        let env = Environment::with_enclosing(self.environment.clone());
        self.execute_block(statements, Rc::new(RefCell::new(env)))
    }

    fn visit_class(&mut self, name: &Token, methods: &Vec<Stmt>) -> Self::Output {
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Value::Nil);

        let mut method_values = HashMap::new();
        for method in methods {
            match method {
                Stmt::Function { name, params, body } => {
                    let function = LoxFunction::new(
                        name,
                        params,
                        body,
                        self.environment.clone(),
                        name.lexeme == "init",
                    );
                    method_values.insert(name.lexeme.clone(), function);
                }
                _ => unreachable!(),
            }
        }

        let class = LoxClass::new(&name.lexeme, method_values).into();
        self.environment.borrow_mut().assign(name, class)?;
        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expr) -> Self::Output {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_function(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Option<Stmt>],
    ) -> Self::Output {
        let function = LoxFunction::new(name, params, body, self.environment.clone(), false);
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Value::LoxFn(function));
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

    fn visit_return(&mut self, keyword: &Token, value: &Option<Rc<Expr>>) -> Self::Output {
        if let Some(value) = value {
            let value = self.evaluate(value)?;
            Err(RuntimeError::new_return(
                keyword.clone(),
                "return".into(),
                Some(value),
            ))
        } else {
            Err(RuntimeError::new_return(
                keyword.clone(),
                "return".into(),
                None,
            ))
        }
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

    fn visit_assign(&mut self, name: &Token, value_expr: &Expr) -> Self::Output {
        let value = self.evaluate(value_expr)?;

        let expr = Expr::new_assign(name.clone(), value_expr.clone()); // hack
        let distance = self.locals.get(&expr);
        if let Some(distance) = distance {
            self.environment
                .borrow_mut()
                .assign_at(*distance, name, &value);
        } else {
            self.globals.borrow_mut().assign(name, value.clone())?;
        }

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
            TokenType::Plus => match (&left, &right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::from(*left + *right)),
                (Value::String(left), Value::String(right)) => {
                    Ok(Value::from(format!("{}{}", left, right)))
                }
                (Value::String(left), _) => {
                    Ok(Value::from(format!("{}{}", left, self.stringify(&right))))
                }
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

    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> Self::Output {
        let callee = self.evaluate(callee)?;

        let mut argument_values = Vec::new();
        for argument in arguments {
            argument_values.push(self.evaluate(argument)?);
        }

        let function: &dyn LoxCallable = match &callee {
            Value::NativeFn(f) => *f,
            Value::LoxFn(f) => f,
            Value::LoxClass(f) => f,
            _ => {
                return Err(RuntimeError::new(
                    paren.clone(),
                    "Can only call functions and classes.".into(),
                ));
            }
        };

        if arguments.len() != function.arity() {
            return Err(RuntimeError::new(
                paren.clone(),
                format!(
                    "Expected {} arguments but got {}.",
                    function.arity(),
                    arguments.len()
                ),
            ));
        }

        function.call(self, &argument_values)
    }

    fn visit_get(&mut self, object: &Expr, name: &Token) -> Self::Output {
        let object = self.evaluate(object)?;
        if let Value::LoxInstance(instance) = object {
            return LoxInstance::get(instance, name);
        }
        Err(RuntimeError::new(
            name.clone(),
            "Only instances have properties.".into(),
        ))
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
        self.look_up_variable(name, &Expr::new_variable(name.clone()))
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

    fn visit_set(&mut self, object: &Expr, name: &Token, value: &Expr) -> Self::Output {
        let object = self.evaluate(object)?;
        if let Value::LoxInstance(instance) = object {
            let value = self.evaluate(value)?;
            instance.borrow_mut().set(name, &value);
            Ok(value)
        } else {
            Err(RuntimeError::new(
                name.clone(),
                "Only instances have fields.".into(),
            ))
        }
    }

    fn visit_this(&mut self, keyword: &Token) -> Self::Output {
        let expr = Expr::new_this(keyword.clone());
        self.look_up_variable(keyword, &expr)
    }
}
