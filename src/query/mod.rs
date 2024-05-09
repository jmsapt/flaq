use crate::tags::FlacTags;
use datetime::{DatePiece, LocalDate, Month, Year, YearMonth};
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest::{iterators::Pairs, RuleType};
use std::{
    io::{self, BufRead},
    num::ParseIntError,
    primitive,
};
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum QueryParseError {
    #[error("Expr::parse expected atom, found `{0}`")]
    AtomError(String),
    #[error("Expr::parse expected infix operation, found `{0}`")]
    InifixError(String),
    #[error("Only NOT (!) is a valid prefix")]
    PrefixError,
    #[error("`{0}` could not be parsed as u32")]
    IntegerError(String),
    #[error("Syntax of query is invalid")]
    SyntaxError,
}

#[derive(Error, Debug)]
pub enum QueryEvalError {
    #[error("Cannot apply logic NOT (!) to type: {0}")]
    InvalidNot(String),
    #[error("Mismatching types cannot be compared: `{0}` and `{0}`")]
    MismatchingTypes(String, String),
    #[error("Invalid String operation: `{0}`")]
    StringOperation(String),
    #[error("Invalid Integer operation: `{0}`")]
    IntegerOperation(String),
    #[error("Invalid Date operation: `{0}`")]
    DateOperation(String),
    #[error("Invalid Boolean operation: `{0}`")]
    BooleanOperation(String),
}

#[derive(pest_derive::Parser)]
#[grammar = "query/query.pest"]
struct QueryParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            // .op(Op::infix(Rule::or, Left))
            .op(Op::infix(Rule::and, Left))
            // .op(Op::prefix(Rule::not, Left))
    };
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
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
    fn eval(&self, lhs: &Expr, rhs: &Expr) -> Result<bool, QueryEvalError> {
        let lhs_val = lhs.eval()?;
        let rhs_val = rhs.eval()?;

        Ok(match (lhs_val, self, rhs_val) {
            // Strings
            (Expr::String(a), Self::Equals, Expr::String(b)) => a == b,
            (Expr::String(a), Self::NotEquals, Expr::String(b)) => a != b,
            (Expr::String(a), Self::Contains, Expr::String(b)) => a.contains(&b),
            (Expr::String(a), op, Expr::String(b)) => {
                Err(QueryEvalError::StringOperation(format!("{op:?}")))?
            }
            // Boolean
            (Expr::Boolean(a), Self::Equals, Expr::Boolean(b)) => a == b,
            (Expr::Boolean(a), Self::NotEquals, Expr::Boolean(b)) => a != b,
            (Expr::Boolean(a), op, Expr::Boolean(b)) => {
                Err(QueryEvalError::BooleanOperation(format!("{op:?}")))?
            }
            // Dates
            (Expr::Date(a), Self::Equals, Expr::Date(b)) => a == b,
            (Expr::Date(a), Self::NotEquals, Expr::Date(b)) => a != b,
            (Expr::Date(a), Self::Greater, Expr::Date(b)) => a > b,
            (Expr::Date(a), Self::GreaterEq, Expr::Date(b)) => a >= b,
            (Expr::Date(a), Self::Less, Expr::Date(b)) => a < b,
            (Expr::Date(a), Self::LessEq, Expr::Date(b)) => a <= b,
            (Expr::Date(a), op, Expr::Date(b)) => {
                Err(QueryEvalError::DateOperation(format!("{op:?}")))?
            }
            // Integer
            (Expr::Integer(a), Self::Equals, Expr::Integer(b)) => a == b,
            (Expr::Integer(a), Self::NotEquals, Expr::Integer(b)) => a != b,
            (Expr::Integer(a), Self::Greater, Expr::Integer(b)) => a > b,
            (Expr::Integer(a), Self::GreaterEq, Expr::Integer(b)) => a >= b,
            (Expr::Integer(a), Self::Less, Expr::Integer(b)) => a < b,
            (Expr::Integer(a), Self::LessEq, Expr::Integer(b)) => a <= b,
            (Expr::Integer(a), op, Expr::Integer(b)) => {
                Err(QueryEvalError::IntegerOperation(format!("{op:?}")))?
            }
            // Type mistmatch
            (a, _, b) => Err(QueryEvalError::MismatchingTypes(
                format!("{a:?}"),
                format!("{b:?}"),
            ))?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    // Operations
    BinOp {
        lhs: Box<Expr<'a>>,
        op: BinaryOperator,
        rhs: Box<Expr<'a>>,
    },
    Not(Box<Expr<'a>>),
    Value(Value<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value<'a> {
    Boolean(bool),
    Date(Date),
    Integer(u32),
    String(&'a str),
    Tag(FlacTags),
}

impl<'a> Expr<'_> {
    pub fn parse_grammer(s: &'a str) -> Result<Pairs<Rule>, QueryParseError> {
        QueryParser::parse(Rule::expr, s).map_err(|_| QueryParseError::SyntaxError)
    }

    pub fn build(pairs: Pairs<Rule>) -> Result<Self, QueryParseError> {
        PRATT_PARSER
            .map_primary(|p| {
                Ok::<Expr, QueryParseError>(match p.as_rule() {
                    Rule::date => todo!(),
                    Rule::string => todo!(),
                    Rule::integer => Expr::Value(Value::Integer(
                        p.as_str()
                            .parse()
                            .map_err(|_| QueryParseError::IntegerError(p.as_str().into()))?,
                    )),
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
            .parse(pairs)
    }

    pub fn eval(&self) -> Result<Box<Expr>, QueryEvalError> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Date {
    Year(i64),
    YearMonth(i64, u8),
    YearMonthDay(i64, u8, u8),
}
