use crate::tags::FlacTags;
use datetime::{DatePiece, LocalDate, Month, Year, YearMonth};
use itertools::Itertools;
use metaflac::block::VorbisComment;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest::{iterators::Pairs, RuleType};
use std::collections::hash_map::Values;
use std::collections::BTreeMap;
use std::ptr::write_bytes;
use std::str::FromStr;
use std::{
    io::{self, BufRead},
    num::ParseIntError,
    primitive,
};
use thiserror::Error;

use crate::PRATT_PARSER;

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
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
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
    #[error("Expression must evaluate to a boolean")]
    BadEvaluation,
}

#[derive(pest_derive::Parser)]
#[grammar = "query/query.pest"]
pub struct QueryParser;
impl QueryParser {
    pub fn parse_grammer(s: &str) -> Result<Pairs<Rule>, QueryParseError> {
        QueryParser::parse(Rule::expr, s).map_err(|_| QueryParseError::SyntaxError)
    }
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
    fn eval(&self, lhs: Value, rhs: Value) -> Result<bool, QueryEvalError> {
        Ok(match (lhs, self, rhs) {
            // Strings
            (Value::String(a), Self::Equals, Value::String(b)) => a == b,
            (Value::String(a), Self::NotEquals, Value::String(b)) => a != b,
            (Value::String(a), Self::Contains, Value::String(b)) => {
                a.iter().any(|x| b.iter().any(|y| x.contains(y)))
            }
            (Value::String(a), op, Value::String(b)) => {
                Err(QueryEvalError::StringOperation(format!("{op:?}")))?
            }
            // Boolean
            (Value::Boolean(a), Self::Equals, Value::Boolean(b)) => a == b,
            (Value::Boolean(a), Self::NotEquals, Value::Boolean(b)) => a != b,
            (Value::Boolean(a), op, Value::Boolean(b)) => {
                Err(QueryEvalError::BooleanOperation(format!("{op:?}")))?
            }
            // Dates
            (Value::Date(a), Self::Equals, Value::Date(b)) => a == b,
            (Value::Date(a), Self::NotEquals, Value::Date(b)) => a != b,
            (Value::Date(a), Self::Greater, Value::Date(b)) => a > b,
            (Value::Date(a), Self::GreaterEq, Value::Date(b)) => a >= b,
            (Value::Date(a), Self::Less, Value::Date(b)) => a < b,
            (Value::Date(a), Self::LessEq, Value::Date(b)) => a <= b,
            (Value::Date(a), op, Value::Date(b)) => {
                Err(QueryEvalError::DateOperation(format!("{op:?}")))?
            }
            // Integer
            (Value::Integer(a), Self::Equals, Value::Integer(b)) => a == b,
            (Value::Integer(a), Self::NotEquals, Value::Integer(b)) => a != b,
            (Value::Integer(a), Self::Greater, Value::Integer(b)) => a > b,
            (Value::Integer(a), Self::GreaterEq, Value::Integer(b)) => a >= b,
            (Value::Integer(a), Self::Less, Value::Integer(b)) => a < b,
            (Value::Integer(a), Self::LessEq, Value::Integer(b)) => a <= b,
            (Value::Integer(a), op, Value::Integer(b)) => {
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
pub enum Expr {
    // Operations
    BinOp {
        lhs: Box<Expr>,
        op: BinaryOperator,
        rhs: Box<Expr>,
    },
    Not(Box<Expr>),
    Value(Value),
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Date(Date),
    Integer(u32),
    String(Vec<String>),
    Tag(FlacTags),
}

pub fn build(pairs: Pairs<Rule>) -> Result<Expr, QueryParseError> {
    PRATT_PARSER
        .map_primary(|p| {
            Ok::<Expr, QueryParseError>(match p.as_rule() {
                Rule::date => Expr::Value(Value::Date(Date::from_str(p.as_str())?)),
                Rule::string => Expr::Value(Value::String(vec![p.as_str().to_string()])),
                Rule::integer => Expr::Value(Value::Integer(
                    p.as_str()
                        .parse()
                        .map_err(|_| QueryParseError::IntegerError(p.as_str().into()))?,
                )),
                Rule::expr => build(p.into_inner())?,
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

impl Expr {
    pub fn eval(&self, env: &VorbisComment) -> Result<Value, QueryEvalError> {
        match self {
            Self::BinOp { lhs, op, rhs } => {
                Value::Boolean(op.eval(lhs.eval(env)?, rhs.eval(env)?)?)
            }
            Self::Not(a) => {
                let v = a.eval(env)?;
                match v {
                    Value::Boolean(b) => v,
                    x => Err(QueryEvalError::InvalidNot(format!("{x:?}")))?,
                }
            }
            Self::Value(v) => match v {
                Value::Tag(t) => match t {
                    FlacTags::Date => todo!(),
                    FlacTags::Tracknumber => todo!(),
                    _ => Value::String(
                        env.get(t.as_str())
                            .map(|v| v.to_owned())
                            .unwrap_or_else(|| Vec::new()),
                    ),
                },
                _ => v.clone(),
            },
        };

        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Date {
    Year(i64),
    YearMonth(i64, u8),
    YearMonthDay(i64, u8, u8),
}
impl FromStr for Date {
    type Err = QueryParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split("-");

        let y = x.next().map(|y| {
            y.parse()
                .map_err(|_| QueryParseError::IntegerError(y.to_string()))
        });
        let m = x.next().map(|y| {
            y.parse()
                .map_err(|_| QueryParseError::IntegerError(y.to_string()))
        });
        let d = x.next().map(|y| {
            y.parse()
                .map_err(|_| QueryParseError::IntegerError(y.to_string()))
        });

        Ok(match (y, m, d) {
            (Some(y), _, _) => Date::Year(y?),
            (Some(y), Some(m), _) => Date::YearMonth(y?, m?),
            (Some(y), Some(m), Some(d)) => Date::YearMonthDay(y?, m?, d?),
            (_, _, _) => Err(QueryParseError::InvalidDate(s.to_string()))?,
        })
    }
}