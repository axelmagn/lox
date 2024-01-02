use std::io::{stdin, Read};

use clap::Parser as _;
use pest::Parser as _;
use rlox2::parser::{LoxParser, Rule};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(long, value_enum, default_value_t = RuleTgt::Program)]
    rule: RuleTgt,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum RuleTgt {
    Program,
    Statement,
    Expression,
}

impl Into<Rule> for RuleTgt {
    fn into(self) -> Rule {
        match self {
            RuleTgt::Program => Rule::program,
            RuleTgt::Statement => Rule::statement,
            RuleTgt::Expression => Rule::expression,
        }
    }
}

pub fn main() {
    let args = Args::parse();
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("cannot read stdin");
    let pairs = LoxParser::parse(args.rule.into(), &input).unwrap();
    for pair in pairs {
        println!("{:#?}", pair);
    }
}
