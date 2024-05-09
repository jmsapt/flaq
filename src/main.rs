// Source document: https://www.xiph.org/vorbis/doc/v-comment.html
#![allow(unused)]

// external crate imports
use clap::Parser;
use colored::Colorize;
use itertools::Itertools;
use metaflac::block::VorbisComment;
use metaflac::Tag;
use query::Expr;
use std::clone;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::option::IntoIter;
use std::path::{Path, PathBuf};
use std::str::FromStr;

// modules
mod cli;
mod operations;
mod query;
mod tags;

// module imports
use cli::{CliArgs, Fields};
use operations::*;
use tags::FlacTags;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let tasks = FlacTags::from_args(args.fields);
    let files = args.arguments.files;

    // handle query
    if let Some(query) = args.arguments.query {
        let grammer = Expr::parse_grammer(&query)?;
        Expr
    }

    if let Some(files) = files {
        // open meta data files
        let mut files = files
            .iter()
            .map(PathBuf::from)
            .map(|p| Tag::read_from_path(p.as_path()).map(|t| (t, p)))
            .collect::<Result<Vec<(_, _)>, _>>()?;

        // apply each edit to every file
        tasks.into_iter().for_each(|(field, tags)| {
            files.iter_mut().for_each(|(tag, path)| {
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
        files.iter_mut().for_each(|(tag, path)| {
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
        files.into_iter().try_for_each(|(mut t, p)| {
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
    }

    Ok(())
}
