use derive_new::new;
use ordered_float::OrderedFloat;
use pest::iterators::Pair;

use crate::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Token<'i> {
    pub pair: Pair<'i, Rule>,
}

impl<'i> From<Pair<'i, Rule>> for Token<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        Self { pair }
    }
}

impl<'i> From<&Pair<'i, Rule>> for Token<'i> {
    fn from(pair: &Pair<'i, Rule>) -> Self {
        Self { pair: pair.clone() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenLiteral {
    Nil,
    String(String),
    Number(OrderedFloat<f64>),
    Bool(bool),
}

impl TokenLiteral {
    pub fn new_nil() -> Self {
        Self::Nil
    }

    pub fn new_string(s: String) -> Self {
        Self::String(s)
    }

    pub fn new_number(n: f64) -> Self {
        Self::Number(OrderedFloat::from(n))
    }

    pub fn new_bool(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl Into<Literal> for TokenLiteral {
    fn into(self) -> Literal {
        Literal::new(self)
    }
}

impl<'i> Into<Expr<'i>> for TokenLiteral {
    fn into(self) -> Expr<'i> {
        Expr::Literal(Literal::new(self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr<'i> {
    Assign(Assign<'i>),
    Binary(Binary<'i>),
    Call(Call<'i>),
    Get(Get<'i>),
    Set(Set<'i>),
    Super(Super<'i>),
    This(This<'i>),
    Grouping(Grouping<'i>),
    Literal(Literal),
    Logical(Logical<'i>),
    Unary(Unary<'i>),
    Variable(Variable<'i>),
}

impl From<f64> for Expr<'_> {
    fn from(value: f64) -> Self {
        TokenLiteral::new_number(value).into()
    }
}

impl From<String> for Expr<'_> {
    fn from(value: String) -> Self {
        TokenLiteral::new_string(value).into()
    }
}

impl From<&str> for Expr<'_> {
    fn from(value: &str) -> Self {
        TokenLiteral::new_string(value.to_owned()).into()
    }
}

impl From<bool> for Expr<'_> {
    fn from(value: bool) -> Self {
        TokenLiteral::new_bool(value).into()
    }
}

impl From<()> for Expr<'_> {
    fn from(_value: ()) -> Self {
        TokenLiteral::Nil.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Assign<'i> {
    pub name: Token<'i>,
    pub value: Box<Expr<'i>>,
}

impl<'i> Into<Expr<'i>> for Assign<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Assign(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Binary<'i> {
    pub left: Box<Expr<'i>>,
    pub operator: Token<'i>,
    pub right: Box<Expr<'i>>,
}

impl<'i> Into<Expr<'i>> for Binary<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Binary(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Call<'i> {
    pub callee: Box<Expr<'i>>,
    pub paren: Token<'i>,
    pub arguments: Vec<Expr<'i>>,
}

impl<'i> Into<Expr<'i>> for Call<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Call(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Get<'i> {
    pub object: Box<Expr<'i>>,
    pub name: Token<'i>,
}

impl<'i> Into<Expr<'i>> for Get<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Get(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Set<'i> {
    pub object: Box<Expr<'i>>,
    pub name: Token<'i>,
    pub value: Box<Expr<'i>>,
}

impl<'i> Into<Expr<'i>> for Set<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Set(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Super<'i> {
    pub keyword: Token<'i>,
    pub method: Token<'i>,
}

impl<'i> Into<Expr<'i>> for Super<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Super(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct This<'i> {
    pub keyword: Token<'i>,
}

impl<'i> Into<Expr<'i>> for This<'i> {
    fn into(self) -> Expr<'i> {
        Expr::This(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Grouping<'i> {
    pub expression: Box<Expr<'i>>,
}

impl<'i> Into<Expr<'i>> for Grouping<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Grouping(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Literal {
    pub value: TokenLiteral,
}

impl<'i> Into<Expr<'i>> for Literal {
    fn into(self) -> Expr<'i> {
        Expr::Literal(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Logical<'i> {
    pub left: Box<Expr<'i>>,
    pub operator: Token<'i>,
    pub right: Box<Expr<'i>>,
}

impl<'i> Into<Expr<'i>> for Logical<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Logical(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Unary<'i> {
    pub operator: Token<'i>,
    pub right: Box<Expr<'i>>,
}

impl<'i> Into<Expr<'i>> for Unary<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Unary(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct Variable<'i> {
    pub name: Token<'i>,
}

impl<'i> Into<Expr<'i>> for Variable<'i> {
    fn into(self) -> Expr<'i> {
        Expr::Variable(self)
    }
}
