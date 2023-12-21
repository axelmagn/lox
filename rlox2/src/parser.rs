use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ast::{self, Expr, TokenLiteral};

#[derive(Parser)]
#[grammar = "lox.pest"]
pub struct LoxParser;

impl LoxParser {
    fn parse_primary(pair: Pair<Rule>) -> Expr {
        let mut inner_pairs = pair.into_inner();
        let inner_pair = inner_pairs
            .next()
            .expect("primary has exactly one inner rule.");
        assert!(
            inner_pairs.next().is_none(),
            "primary has exactly one inner rule."
        );

        let token = (&inner_pair).into();

        match inner_pair.as_rule() {
            Rule::TRUE => true.into(),
            Rule::FALSE => false.into(),
            Rule::NUMBER => inner_pair
                .as_str()
                .parse::<f64>()
                .expect("could not parse number.")
                .into(),
            Rule::NIL => TokenLiteral::new_nil().into(),
            Rule::THIS => ast::This::new(token).into(),
            Rule::IDENTIFIER => ast::Variable::new(token).into(),
            Rule::grouping => Self::parse_grouping(inner_pair),
            Rule::super_term => Self::parse_super_term(inner_pair),
            _ => unreachable!(),
        }
    }

    fn parse_grouping(inner_pair: Pair<'_, Rule>) -> Expr<'_> {
        todo!()
    }

    fn parse_super_term(inner_pair: Pair<'_, Rule>) -> Expr<'_> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_primary() {
        fn check(s: &str) -> Expr {
            let mut pairs = LoxParser::parse(Rule::primary, s).unwrap();
            let pair = pairs.next().expect("no pairs parsed.");
            assert!(pairs.next().is_none());
            LoxParser::parse_primary(pair)
        }
        assert_eq!(check("true"), true.into());
        assert_eq!(check("false"), false.into());
        assert_eq!(check("42.65"), 42.65.into());
        assert_eq!(check("nil"), ().into());
        assert!(match check("this") {
            Expr::This(_) => true,
            _ => false,
        });
        assert!(match check("foobar") {
            Expr::Variable(_) => true,
            _ => false,
        });

        // todo: grouping, super
    }
}
