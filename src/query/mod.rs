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
    #[error("Tag cannot be compared as it is not set: {0}")]
    TagNotSet(String),
}

#[derive(pest_derive::Parser)]
#[grammar = "query/query.pest"]
pub struct QueryParser;
impl QueryParser {
    pub fn parse_grammer(s: &str) -> Result<Pairs<Rule>, QueryParseError> {
        QueryParser::parse(Rule::expr, s).map_err(|_| QueryParseError::SyntaxError)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            (Value::String(a), Self::Equals, Value::String(b)) => {
                a.iter().any(|x| b.iter().any(|y| x == y))
            }
            (Value::String(a), Self::NotEquals, Value::String(b)) => a != b,
            (Value::String(a), Self::Contains, Value::String(b)) => a.iter().any(|x| {
                b.iter()
                    .any(|y| x.to_lowercase().contains(y.to_lowercase().as_str()))
            }),
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
                Rule::string => Expr::Value(Value::String(vec![p
                    .as_str()
                    .strip_prefix('\"')
                    .unwrap()
                    .strip_suffix('\"')
                    .unwrap()
                    .to_string()])),
                Rule::integer => Expr::Value(Value::Integer(
                    p.as_str()
                        .parse()
                        .map_err(|_| QueryParseError::IntegerError(p.as_str().into()))?,
                )),
                Rule::tag => Expr::Value(Value::Tag(
                    FlacTags::from_str(p.as_str()).expect("Tag validated by pest grammar already"),
                )),
                Rule::expr => build(p.into_inner())?,
                _ => Err(QueryParseError::AtomError(p.as_str().into()))?,
            })
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::equals => BinaryOperator::Equals,
                Rule::not_equals => BinaryOperator::NotEquals,
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
        Ok(match self {
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
                    FlacTags::Date => Value::Date(
                        Date::from_str(
                            env.get(t.as_str())
                                .ok_or(QueryEvalError::TagNotSet(format!("{t:?}")))?
                                .iter()
                                .next()
                                .ok_or(QueryEvalError::TagNotSet(format!("{t:?}")))?
                                .as_str(),
                        )
                        .map_err(|_| QueryEvalError::DateOperation(format!("{t:?}")))?,
                    ),
                    FlacTags::Tracknumber => Value::Integer(
                        env.get(t.as_str())
                            .ok_or(QueryEvalError::TagNotSet(format!("{t:?}")))?
                            .iter()
                            .next()
                            .ok_or(QueryEvalError::TagNotSet(format!("{t:?}")))?
                            .as_str()
                            .parse()
                            .map_err(|_| QueryEvalError::IntegerOperation(format!("{t:?}")))?,
                    ),
                    _ => Value::String(
                        env.get(t.as_str())
                            .map(|v| v.to_owned())
                            .ok_or(QueryEvalError::TagNotSet(format!("{t:?}")))?,
                    ),
                },
                _ => v.clone(),
            },
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Date {
    Year(u32),
    YearMonth(u32, u8),
    YearMonthDay(u32, u8, u8),
}
impl From<u32> for Date {
    fn from(value: u32) -> Self {
        Date::Year(value)
    }
}
impl From<Date> for u32 {
    fn from(value: Date) -> Self {
        match value {
            Date::Year(year) => year,
            Date::YearMonth(year, _) => year,
            Date::YearMonthDay(year, _, _) => year,
        }
    }
}
impl FromStr for Date {
    type Err = QueryParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split('-');

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

#[cfg(test)]
mod test_parsing {
    use super::*;

    fn expr(query: &str) -> Expr {
        let grammer = dbg!(QueryParser::parse_grammer(query)).unwrap();
        build(grammer).unwrap()
    }

    fn env() -> VorbisComment {
        let mut v = VorbisComment::new();

        v.set("TITLE", vec!["Feather"]);
        v.set("ARTIST", vec!["Nujabes", "Cise Starr"]);
        v.set("DATE", vec!["2005"]);

        v
    }

    macro_rules! assert_val {
        ($val:expr, $x:expr) => {{
            match $val {
                Value::Boolean(b) => assert_eq!(b, $x),
                _ => panic!("Not a boolean value"),
            }
        }};
    }

    #[test]
    fn const_expr_1() {
        let query = stringify!("Foo" == "Foo");
        let expr_exp = Expr::BinOp {
            lhs: Box::new(Expr::Value(Value::String(vec!["Foo".to_string()]))),
            op: BinaryOperator::Equals,
            rhs: Box::new(Expr::Value(Value::String(vec!["Foo".to_string()]))),
        };

        let expr_act = expr(query);
        assert_eq!(expr_exp, expr_act);
        assert_val!(expr_act.eval(&env()).unwrap(), true);
    }

    #[test]
    fn const_expr_2() {
        let query = stringify!(10 == 10);
        let expr_exp = Expr::BinOp {
            lhs: Box::new(Expr::Value(Value::Integer(10))),
            op: BinaryOperator::Equals,
            rhs: Box::new(Expr::Value(Value::Integer(10))),
        };

        let expr_act = expr(query);
        assert_eq!(expr_exp, expr_act);
        assert_val!(expr_act.eval(&env()).unwrap(), true);
    }
    // fix date formats
    // #[test]
    // fn const_expr_3() {
    //     let query = stringify!(1999 - 01 - 01 == 1999 - 01 - 01);
    //     let expr_exp = Expr::BinOp {
    //         lhs: Box::new(Expr::Value(Value::Date(Date::YearMonthDay(1999, 1, 1)))),
    //         op: BinaryOperator::Equals,
    //         rhs: Box::new(Expr::Value(Value::Date(Date::YearMonthDay(1999, 1, 1)))),
    //     };

    //     let expr_act = expr(query);
    //     assert_eq!(expr_exp, expr_act);
    //     assert_val!(expr_act.eval(&env()).unwrap(), true);
    // }
    #[test]
    fn const_expr_4() {
        let query = stringify!(1 != 2);
        let expr_exp = Expr::BinOp {
            lhs: Box::new(Expr::Value(Value::Integer(1))),
            op: BinaryOperator::NotEquals,
            rhs: Box::new(Expr::Value(Value::Integer(2))),
        };

        let expr_act = expr(query);
        assert_eq!(expr_exp, expr_act);
        assert_val!(expr_act.eval(&env()).unwrap(), true);
    }
    #[test]
    fn const_expr_5() {
        let query = stringify!("Foo" == "Bar");
        let expr_exp = Expr::BinOp {
            lhs: Box::new(Expr::Value(Value::String(vec!["Foo".to_string()]))),
            op: BinaryOperator::Equals,
            rhs: Box::new(Expr::Value(Value::String(vec!["Bar".to_string()]))),
        };

        let expr_act = expr(query);
        assert_eq!(expr_exp, expr_act);
        assert_val!(expr_act.eval(&env()).unwrap(), false);
    }
}
