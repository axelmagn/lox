use crate::{
    expr::Expr,
    lox::Lox,
    stmt::Stmt,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

struct ParseError;

impl Parser<'_> {
    pub fn new<'a>(tokens: &'a [Token]) -> Parser<'a> {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Option<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let stmt: Result<Stmt, ParseError> = if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match stmt {
            Ok(v) => Some(v),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_stmt();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Stmt::new_block(self.block()?));
        }

        self.expression_stmt()
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::new_print(value))
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();
        let mut initializer = None;
        if self.match_token(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::new_var(name.clone(), initializer))
    }

    fn expression_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::new_expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Option<Stmt>>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::new_literal_bool(false));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::new_literal_bool(true));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::new_literal_nil());
        }
        if self.match_token(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::new_literal(self.previous().literal.clone()));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::new_grouping(expr));
        }
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::new_variable(self.previous().clone()));
        }

        Err(self.error(self.peek(), "Expect expression"))
    }

    fn match_token(&mut self, ttypes: &[TokenType]) -> bool {
        for ttype in ttypes {
            if self.check(*ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), msg))
        }
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ttype == ttype
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, msg: &str) -> ParseError {
        Lox::error_on_token(token, msg);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }
        }

        match self.peek().ttype {
            TokenType::Class
            | TokenType::Fun
            | TokenType::Var
            | TokenType::For
            | TokenType::If
            | TokenType::While
            | TokenType::Print
            | TokenType::Return => {
                return;
            }
            _ => {}
        };

        self.advance();
    }
}
