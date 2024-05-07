use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use std::{collections::HashMap, str::FromStr};
use strum::ParseError;
use thiserror::Error;

use crate::tags::FlacTags;

// cursed macro to make unwrapping slightly less bad (o_o)
// this should be not panic as long as the `pest` code
// is sound
macro_rules! next {
    () => {
        next().unwrap()
    };
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid tag `{0}`")]
    TagError(&'static str),
}

#[derive(Parser)]
#[grammar = "tags.pest"]
pub struct TagParser;
impl TagParser {
    pub fn eval(tag_string: &str) -> Result<HashMap<FlacTags, Vec<&str>>, String> {
        let mut map = HashMap::new();

        match Self::parse(Rule::key_value, tag_string) {
            Ok(mut tags) => tags.try_for_each(|p| {
                let mut inner = dbg!(p.into_inner());
                let key = FlacTags::from_str(
                    inner.next().unwrap().into_inner().next().unwrap().as_str(),
                )?;
                let value_node = inner.next().unwrap();

                // handle both cases
                let values = match value_node.as_rule() {
                    Rule::value => value_node
                        .into_inner()
                        .map(|s| s.into_inner().as_str())
                        .collect::<Vec<_>>(),
                    // Rule::string => vec![dbg!(value_node.into_inner().next().unwrap().as_str())],
                    e => {
                        dbg!(e);
                        todo!();
                    }
                };

                map.insert(key, values);
                Ok::<(), String>(())
            })?,
            Err(e) => {
                dbg!(e);
                todo!();
            }
        };

        Ok(map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_parse_string() {
        use FlacTags::*;
        // TODO: validate the resulting hashmap, not just ok/err
        let valid = vec![
            ("title:bar", HashMap::from([(Title, vec!["bar"])])),
            ("title:\"bar\"", HashMap::from([(Title, vec!["bar"])])),
            (" title : bar   ", HashMap::from([(Title, vec!["bar"])])),
            ("title:(bar,foo)", HashMap::from([(Title, vec!["bar", "foo"])])),
            ("title:(bar, foo)", HashMap::from([(Title, vec!["bar", "foo"])])),
            ("title:(bar, foo, Bam,)", HashMap::from([(Title, vec!["bar", "foo", "Bam"])])),
            ("title : ( bar , foo )", HashMap::from([(Title, vec!["bar", "foo"])])),
            ("title:(bar, foo,)", HashMap::from([(Title, vec!["bar", "foo"])])),
            ("title:bar artisit:bar", HashMap::from([(Title, vec!["bar"]),  (Title, vec!["bar"])])),
        ];
        let invalid = vec![
            // "invalid"
        ];

        valid
            .into_iter()
            .for_each(|(input, map)| {
                assert_eq!(TagParser::eval(dbg!(input)).unwrap(), map)
            });
        invalid
            .into_iter()
            .for_each(|i| assert!(TagParser::eval(dbg!(i)).is_err()));
    }
}
