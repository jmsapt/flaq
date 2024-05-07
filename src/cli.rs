use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[clap(
    // override_usage = "flaq [FLAGS] [OPTIONS] [ARGUMENTS]",
    about = "Short about - ToDO",
    long_about = "TODO",
    version,
    author = "James Appleton",
    disable_version_flag = true,
)]
pub struct CliArgs {
    /// Print version
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    pub version_number: (),

    /// Clean duplicated fields, where both the tag field and value match.
    ///
    /// Runs after all edits (if used alongside edits).
    #[clap(long, short, conflicts_with_all = &["clean_all"], action = clap::ArgAction::SetTrue)]
    pub clean: bool,

    /// Clean duplicated fields, and deletes non-standard header comments.
    ///
    /// Runs after all edits (if used alongside edits).
    #[clap(long, short = 'C', conflicts_with_all = &["clean"], action = clap::ArgAction::SetFalse)]
    pub clean_all: bool,

    /// [Default] Set field to new given values. Previous values are deleted.
    #[clap(long, conflicts_with_all = &["append", "delete"], action = clap::ArgAction::SetTrue)]
    pub set: bool,

    /// Append flag. If set appends values leaving existing values untouched.
    #[clap(long, conflicts_with_all = &["set", "delete"], action = clap::ArgAction::SetFalse)]
    pub append: bool,

    /// Deletes associated values for provided fields, leaving the field unset.
    #[clap(long, conflicts_with_all = &["append", "set"], action = clap::ArgAction::SetFalse)]
    pub delete: bool,

    /// Deletes associated values for provided fields, leaving the field unset.
    #[clap(long, short = 'L', conflicts_with_all = &["list"], action = clap::ArgAction::SetFalse)]
    pub list_detailed: bool,

    /// Deletes associated values for provided fields, leaving the field unset.
    #[clap(long, short = 'l', conflicts_with_all = &["list_detailed"], action = clap::ArgAction::SetFalse)]
    pub list: bool,

    /// Deletes associated values for provided fields, leaving the field unset.
    #[clap(long, action = clap::ArgAction::SetFalse)]
    pub dry_run: bool,

    #[command(flatten)]
    pub fields: Fields,

    #[command(flatten)]
    pub arguments: Arguments,
}

#[derive(Debug, Hash, PartialEq, Eq, Args, Clone)]
#[clap(next_help_heading = "Arguments")]
pub struct Arguments {
    #[clap(short, long)]
    pub query: Option<String>,

    #[clap(short, long, value_hint=clap::ValueHint::FilePath)]
    #[arg(num_args(0..))]
    pub files: Option<Vec<String>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Args, Clone)]
#[clap(next_help_heading = "Tag Fields")]
pub struct Fields {
    #[clap(short, long, num_args(0..))]
    pub title: Option<Vec<String>>,

    #[clap(long = "song-version", num_args(0..))]
    pub version: Option<Vec<String>>,

    #[clap(short = 'A', long, num_args(0..))]
    pub album: Option<Vec<String>>,

    #[clap(short = 'n', long,num_args(0..))]
    pub tracknumber: Option<Vec<String>>,

    #[clap(short = 'a', long,num_args(0..))]
    pub artist: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub performer: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub copyright: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub license: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub organization: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub description: Option<Vec<String>>,

    #[clap(short, long, num_args(0..))]
    pub genre: Option<Vec<String>>,

    #[clap(short, long, num_args(0..))]
    pub date: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub location: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub contact: Option<Vec<String>>,

    #[clap(long, num_args(0..))]
    pub isrc: Option<Vec<String>>,
}
