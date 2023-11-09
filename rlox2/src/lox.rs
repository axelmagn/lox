use crate::parser::{LoxParser, Rule};

use pest::Parser;

/// Entrypoint for the lox interpreter.
pub struct Lox;

impl Lox {
    fn run(source: &str) {
        // todo: error handling
        let program = LoxParser::parse(Rule::program, source)
            .unwrap()
            .next()
            .unwrap();
    }
}
