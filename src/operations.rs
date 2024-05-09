use crate::tags::FlacTags;
use colored::Colorize;
use itertools::Itertools;
use metaflac::block::VorbisComment;
use std::path::Path;
use std::str::FromStr;

/// Sets tags for the given field.
pub fn set_tags(meta: &mut VorbisComment, field: FlacTags, tags: Vec<String>) {
    meta.set(field.as_str(), tags)
}

/// Append tags to the given field.
pub fn append_tags(meta: &mut VorbisComment, field: FlacTags, mut tags: Vec<String>) {
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
pub fn delete_tags(meta: &mut VorbisComment, field: FlacTags) {
    meta.remove(field.as_str())
}

/// Removes duplicated tags
pub fn clean_tags(meta: &mut VorbisComment) {
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
pub fn clean_non_standard_tags(meta: &mut VorbisComment) {
    let bad_tags = meta
        .comments
        .keys()
        .filter(|k| FlacTags::from_str(k.as_str()).is_err())
        .map(|t| t.to_owned())
        .collect::<Vec<_>>();

    bad_tags.into_iter().for_each(|t| meta.remove(&t))
}

/// Print a newline seperate list of files, ready for piping into other programs.
pub fn list(path: &Path) {
    println!("{}", path.to_str().unwrap())
}

/// Prints in a more human readable format, listing all files and each tag on that file.
/// Prints what the edit would be, prior them actually being commit (useful for a dry run).
pub fn list_detailed(path: &Path, tag: &VorbisComment) {
    println!("{:6} {}", "File:".bold(), path.to_str().unwrap().green());
    println!("{}", "Tags:".bold());
    tag.comments.keys().sorted().for_each(|k| {
        println!("{:6}{}:{:?}", "", k.as_str().blue(), tag.comments[k]);
    });
}
