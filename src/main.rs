// Source document: https://www.xiph.org/vorbis/doc/v-comment.html
#![allow(unused)]
use clap::Parser;
use cli::{CliArgs, Fields};
use colored::Colorize;
use itertools::Itertools;
use metaflac::block::VorbisComment;
use metaflac::Tag;
use std::clone;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::option::IntoIter;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tags::FlacTags;

macro_rules! field {
    ($vec:expr, $field:expr, $enum:expr) => {{
        if let Some(v) = $field {
            $vec.push(($enum, v));
        }
    }};
}

mod cli;
mod tags;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let tasks = FlacTags::from_args(args.fields);

    // TODO: handle query
    let files = args.arguments.files;

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
        });
    }

    Ok(())
}

/// Sets tags for the given field.
fn set_tags(meta: &mut VorbisComment, field: FlacTags, tags: Vec<String>) {
    meta.set(field.as_str(), tags)
}

/// Append tags to the given field.
fn append_tags(meta: &mut VorbisComment, field: FlacTags, mut tags: Vec<String>) {
    let curr_tags = meta.get(field.as_str());

    let new_tags = if let Some(curr_tags) = curr_tags {
        let mut new_tags = curr_tags.to_owned();
        new_tags.append(&mut tags);
        new_tags
    } else {
        tags
    };

    meta.set(field.as_str(), new_tags);
}

/// Deletes tags for given field
fn delete_tags(meta: &mut VorbisComment, field: FlacTags) {
    meta.remove(field.as_str())
}

/// Removes duplicated tags
fn clean_tags(meta: &mut VorbisComment) {
    let new_values = meta
        .comments
        .iter()
        .map(|(k, v)| {
            (
                k.to_owned(),
                v.iter().map(|s| s.to_owned()).unique().collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    new_values.into_iter().for_each(|(k, v)| {
        meta.set(k, v);
    });
}

/// Deletes all non-standard tags.
fn clean_non_standard_tags(meta: &mut VorbisComment) {
    let bad_tags = meta
        .comments
        .keys()
        .filter(|k| FlacTags::from_str(k.as_str()).is_err())
        .map(|t| t.to_owned())
        .collect::<Vec<_>>();

    bad_tags.into_iter().for_each(|t| meta.remove(&t))
}

/// Print a newline seperate list of files, ready for piping into other programs.
fn list(path: &Path) {
    println!("{}", path.to_str().unwrap())
}

/// Prints in a more human readable format, listing all files and each tag on that file.
/// Prints what the edit would be, prior them actually being commit (useful for a dry run).
fn list_detailed(path: &Path, tag: &VorbisComment) {
    println!("{:6} {}", "File:".bold(), path.to_str().unwrap().green());
    println!("{}", "Tags:".bold());
    tag.comments.keys().sorted().for_each(|k| {
        println!("{:6}{}:{:?}", "", k.as_str().blue(), tag.comments[k]);
    });
}
