use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lox.pest"]
pub struct LoxParser;
