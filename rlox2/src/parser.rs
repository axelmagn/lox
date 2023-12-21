use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lox.pest"]
pub struct LoxParser;

mod ast_parser {
    use super::*;
    use crate::ast::{self, Expr, TokenLiteral};
    use pest::iterators::Pair;

    fn expression(pair: Pair<Rule>) -> Expr {
        todo!();
    }

    fn assignment(pair: Pair<Rule>) -> Expr {
        let mut inner = pair.into_inner();
        let mut cursor = inner.next().unwrap();

        if let Rule::logic_or = &cursor.as_rule() {
            return logic_or(cursor);
        }

        while let Rule::call = &cursor.as_rule() {}
        todo!();
    }

    fn logic_or(pair: Pair<Rule>) -> Expr {
        todo!();
    }

    fn primary(pair: Pair<Rule>) -> Expr {
        let mut inner_pairs = pair.into_inner();
        let inner_pair = inner_pairs.next().unwrap();
        assert!(inner_pairs.next().is_none());

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
            Rule::THIS => ast::This::new().into(),
            Rule::IDENTIFIER => ast::Variable::new(token).into(),
            Rule::grouping => grouping(inner_pair),
            Rule::super_term => super_term(inner_pair),
            _ => unreachable!(),
        }
    }

    fn grouping(pair: Pair<'_, Rule>) -> Expr<'_> {
        let mut inner = pair.into_inner();
        let inner_pair = inner.next().unwrap();
        assert!(inner.next().is_none());
        assert_eq!(inner_pair.as_rule(), Rule::expression);
        let expression = expression(inner_pair).into();
        ast::Grouping::new(expression).into()
    }

    fn super_term(pair: Pair<'_, Rule>) -> Expr<'_> {
        let mut inner = pair.into_inner();
        let method = inner.next().unwrap();
        assert!(inner.next().is_none());
        assert_eq!(method.as_rule(), Rule::IDENTIFIER);
        ast::Super::new(method.into()).into()
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
                primary(pair)
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
            assert!(match check("super.foobar") {
                Expr::Super(_) => true,
                _ => false,
            });
            assert!(match check("(42.65 + 6)") {
                Expr::Grouping(_) => true,
                _ => false,
            });
        }
    }
}
