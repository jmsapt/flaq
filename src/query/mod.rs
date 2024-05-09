use crate::tags::FlacTags;
use datetime::{LocalDate, Year, YearMonth};
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use std::{
    io::{self, BufRead},
    num::ParseIntError,
    primitive,
};
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
enum QueryParseError {
    #[error("Expr::parse expected atom, found `{0}`")]
    AtomError(String),
    #[error("Expr::parse expected infix operation, found `{0}`")]
    InifixError(String),
    #[error("Only NOT (!) is a valid prefix")]
    PrefixError,
    #[error("`{0}` could not be parsed as u32")]
    IntegerError(String),
}

#[derive(Error, Debug)]
enum QueryEvalError {
    #[error("Cannot apply logic NOT (!) to type: {0}")]
    InvalidNot(String),
    #[error("Mismatching types cannot be compared: `{0}` and `{0}`")]
    MismatchingTypes(String, String),
    #[error("Invalid String operation: `{0}`")]
    StringOperation(String),
}

#[derive(pest_derive::Parser)]
#[grammar = "query/query.pest"]
pub struct QueryParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            // .op(Op::infix(Rule::or, Left))
            .op(Op::infix(Rule::and, Left))
            // .op(Op::prefix(Rule::not, Left))
    };
}

#[derive(Debug)]
enum BinaryOperator {
    Equals,
    NotEquals,
    Contains,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    And,
    Or,
}
impl BinaryOperator {
    fn eval(&self, lhs: Expr, rhs: Expr) -> Result<bool, QueryEvalError> {
        let lhs_val = lhs.eval()?;
        let rhs_val = rhs.eval()?;

        match (lhs_val, self, rhs_val) {
            // Strings
            (Expr::String(a), Self::Equals, Expr::String(b)) => todo!(),
            (Expr::String(a), Self::NotEquals, Expr::String(b)) => todo!(),
            (Expr::String(a), _, Expr::String(b)) => todo!(),
            // Boolean
            (Expr::Boolean(a), _, Expr::Boolean(b)) => todo!(),
            (Expr::Boolean(a), _, Expr::Boolean(b)) => todo!(),
            (Expr::Boolean(a), _, Expr::Boolean(b)) => todo!(),
            // Dates
            (Expr::Date(a), Self::Equals, Expr::Date(b)) => todo!(),
            (Expr::Date(a), Self::NotEquals, Expr::Date(b)) => todo!(),
            (Expr::Date(a), Self::Greater, Expr::Date(b)) => todo!(),
            (Expr::Date(a), Self::GreaterEq, Expr::Date(b)) => todo!(),
            (Expr::Date(a), Self::Less, Expr::Date(b)) => todo!(),
            (Expr::Date(a), Self::LessEq, Expr::Date(b)) => todo!(),
            (Expr::Date(a), _, Expr::Date(b)) => todo!(),
            // Integer
            (Expr::Integer(a), Self::Equals, Expr::Integer(b)) => todo!(),
            (Expr::Integer(a), Self::NotEquals, Expr::Integer(b)) => todo!(),
            (Expr::Integer(a), Self::Greater, Expr::Integer(b)) => todo!(),
            (Expr::Integer(a), Self::GreaterEq, Expr::Integer(b)) => todo!(),
            (Expr::Integer(a), Self::Less, Expr::Integer(b)) => todo!(),
            (Expr::Integer(a), Self::LessEq, Expr::Integer(b)) => todo!(),
            (Expr::Integer(a), _, Expr::Integer(b)) => todo!(),
            // Type mistmatch
            (a, _, b) => Err(QueryEvalError::MismatchingTypes(
                format!("{a:?}"),
                format!("{b:?}"),
            )),
        }
    }
}

#[derive(Debug)]
enum Expr {
    // Operations
    BinOp {
        lhs: Box<Expr>,
        op: BinaryOperator,
        rhs: Box<Expr>,
    },
    Not(Box<Expr>),
    Boolean(bool),
    Date(Date),
    Integer(u32),
    String(String),
    Tag(FlacTags),
}
impl Expr {
    pub fn build(pairs: Pairs<Rule>) -> Result<Self, QueryParseError> {
        let query = PRATT_PARSER
            .map_primary(|p| {
                Ok::<Expr, QueryParseError>(match p.as_rule() {
                    Rule::date => todo!(),
                    Rule::string => todo!(),
                    Rule::integer => Expr::Integer(
                        p.as_str()
                            .parse()
                            .map_err(|_| QueryParseError::IntegerError(p.as_str().into()))?,
                    ),
                    Rule::expr => Expr::build(p.into_inner())?,
                    _ => Err(QueryParseError::AtomError(p.as_str().into()))?,
                })
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::equals => BinaryOperator::Equals,
                    Rule::contains => BinaryOperator::Contains,
                    Rule::greater => BinaryOperator::Greater,
                    Rule::greater_eq => BinaryOperator::GreaterEq,
                    Rule::less => BinaryOperator::Less,
                    Rule::less_eq => BinaryOperator::LessEq,
                    Rule::and => BinaryOperator::And,
                    Rule::or => BinaryOperator::Or,
                    rule => Err(QueryParseError::InifixError(op.as_str().into()))?,
                };

                Ok(Expr::BinOp {
                    lhs: Box::new(lhs?),
                    op,
                    rhs: Box::new(rhs?),
                })
            })
            .map_prefix(|op, rhs| {
                Ok(match op.as_rule() {
                    Rule::not => Expr::Not(Box::new(rhs?)),
                    _ => Err(QueryParseError::PrefixError)?,
                })
            })
            .parse(pairs);
        todo!()
    }

    pub fn eval(self) -> Result<Expr, QueryEvalError> {
        Ok(match self {
            Expr::BinOp { lhs, op, rhs } => Expr::Boolean(op.eval(*lhs, *rhs)?),
            Expr::Not(operand) => match operand.eval()? {
                Expr::Boolean(b) => Expr::Boolean(!b),
                p => Err(QueryEvalError::InvalidNot(format!("{p:?}")))?,
            },
            p => p,
        })
    }
}

#[derive(Debug)]
enum Date {
    Year(Year),
    YearMonth(YearMonth),
    YearMonthDay(LocalDate),
}
