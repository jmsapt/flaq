use crate::tags::FlacTags;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use std::{
    io::{self, BufRead},
    num::ParseIntError,
};
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
enum QueryErrors {
    #[error("Expr::parse expected atom, found `{0}`")]
    AtomError(String),
    #[error("Expr::parse expected infix operation, found `{0}`")]
    InifixError(String),
    #[error("Only NOT (!) is a valid prefix")]
    PrefixError,
    #[error("`{0}` could not be parsed as u32")]
    IntegerError(String),
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

#[derive(Debug)]
enum Expr {
    // Operations
    BinOp {
        lhs: Box<Expr>,
        op: BinaryOperator,
        rhs: Box<Expr>,
    },
    Not(Box<Expr>),

    // Literals
    Date(Date),
    Integer(u32),
    String(String),
    Tag(FlacTags),
}
impl Expr {
    pub fn build(pairs: Pairs<Rule>) -> Result<Self, QueryErrors> {
        let query = PRATT_PARSER
            .map_primary(|p| {
                Ok::<Expr, QueryErrors>(match p.as_rule() {
                    Rule::date => todo!(),
                    Rule::string => todo!(),
                    Rule::integer => Expr::Integer(
                        p.as_str()
                            .parse()
                            .map_err(|_| QueryErrors::IntegerError(p.as_str().into()))?,
                    ),
                    Rule::expr => Expr::build(p.into_inner())?,
                    _ => Err(QueryErrors::AtomError(p.as_str().into()))?,
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
                    rule => Err(QueryErrors::InifixError(op.as_str().into()))?,
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
                    _ => Err(QueryErrors::PrefixError)?,
                })
            })
            .parse(pairs);
        todo!()
    }

    fn evaluate() -> bool {
        //
        todo!()
    }
}

#[derive(Debug)]
enum Date {
    Year,
    YearMonth,
    YearMonthDay,
}
