// Source document: https://www.xiph.org/vorbis/doc/v-comment.html
#![allow(unused)]

use clap::error::Result;
// external crate imports
use clap::Parser;
use colored::Colorize;
use itertools::Itertools;
use metaflac::block::VorbisComment;
use metaflac::Tag;
use pest::pratt_parser::PrattParser;
use query::{build, Expr, QueryParser, Value};
use query::{QueryEvalError, QueryParseError, Rule};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::ReadDir;
use std::option::IntoIter;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{clone, fs};
// modules
mod cli;
mod operations;
mod query;
mod tags;

// module imports
use cli::{CliArgs, Fields};
use operations::*;
use tags::FlacTags;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            // .op(Op::infix(Rule::or, Left))
            .op(Op::infix(Rule::and, Left))
            // .op(Op::prefix(Rule::not, Left))
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let tasks = FlacTags::from_args(args.fields);
    let mut paths = Vec::new();

    if let Some(files) = args.arguments.files {
        files.iter().map(PathBuf::from).for_each(|p| paths.push(p));
    }

    // handle query
    if let Some(query_str) = args.arguments.query {
        // build expression
        let grammer = QueryParser::parse_grammer(&query_str)?;
        let expression = build(grammer)?;

        // evaulate against all files recursively
        let mut buffer = Vec::new();
        let cwd = std::fs::read_dir("./").unwrap();
        get_paths(cwd, &mut buffer);

        buffer
            .into_iter()
            // evaluate expressions
            .filter_map(|p| {
                let t = Tag::read_from_path(p.as_path()).ok()?;
                let env = t.vorbis_comments()?;
                Some(expression.eval(env).map(|v| (v, p)))
            })
            // catch any errors
            .collect::<Result<Vec<(Value, PathBuf)>, QueryEvalError>>()?
            .into_iter()
            .map(|(v, p)| match v {
                Value::Boolean(b) => Ok((b, p)),
                _ => Err(QueryEvalError::BadEvaluation)?,
            })
            // catch any invalidate types
            .collect::<Result<Vec<_>, QueryEvalError>>()?
            .into_iter()
            .filter_map(|(v, p)| if v { Some(p) } else { None })
            // append matching queries' paths
            .for_each(|p| paths.push(p));
    };

    // open meta data files
    let mut paths = paths
        .iter()
        .map(|p| Tag::read_from_path(p.as_path()).map(|t| (t, p)))
        .collect::<Result<Vec<(_, _)>, _>>()?;

    // apply each edit to every file
    tasks.into_iter().for_each(|(field, tags)| {
        paths.iter_mut().for_each(|(tag, path)| {
            let mut meta = tag.vorbis_comments_mut();
            // Set, append, and delete are parsed as mutually exclusive
            if dbg!(args.set) {
                /* set */
                set_tags(meta, field, dbg!(tags.to_owned()));
            } else if args.append {
                /* append */
                append_tags(meta, field, tags.to_owned());
            } else if args.delete {
                /* delete */
                delete_tags(meta, field);
            }
        });
    });

    // optionally performing cleaning
    paths.iter_mut().for_each(|(tag, path)| {
        let mut meta = tag.vorbis_comments_mut();
        if args.clean {
            /* clean */
            clean_tags(meta);
        } else if args.clean_all {
            /* clean non-standard tags */
            clean_non_standard_tags(meta);
        }
    });

    // print listing information and save
    paths.into_iter().try_for_each(|(mut t, p)| {
        if args.list {
            /* print filenames */
            list(p.as_path());
        } else if args.list_detailed {
            /* print filename & all current tags */
            list_detailed(p.as_path(), t.vorbis_comments_mut());
        }

        /* save meta data */
        if !args.dry_run {
            t.save()?;
        }

        Ok::<(), metaflac::Error>(())
    })?;

    Ok(())
}

fn get_paths(dir: ReadDir, buffer: &mut Vec<PathBuf>) {
    for path in dir {
        let p = path.unwrap().path();

        if p.is_dir() {
            get_paths(fs::read_dir(p.as_path()).unwrap(), buffer)
        } else {
            buffer.push(p)
        }
    }
}
