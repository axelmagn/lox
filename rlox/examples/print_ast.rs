use rlox::ast_printer::AstPrinter;
use rlox::expr::Expr;
use rlox::token::{Token, TokenLiteral, TokenType};

pub fn main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-", &TokenLiteral::None, 1),
            right: Box::new(Expr::Literal {
                value: TokenLiteral::Number(123.),
            }),
        }),
        operator: Token::new(TokenType::Star, "*", &TokenLiteral::None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: TokenLiteral::Number(45.67),
            }),
        }),
    };
    println!("{}", AstPrinter::new().print(&expression));
}
