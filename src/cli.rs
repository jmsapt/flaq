use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[clap(
    about = "A CLI tool for editing and query `.flac` files metadata tags",
    // long_about = "TODO",
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
    #[clap(long, short, conflicts_with_all = &["clean_all"], action)]
    pub clean: bool,

    /// [Caution] Clean duplicated fields, and deletes ALL non-standard header comments.
    ///
    /// Runs after all edits (if used alongside edits). If the previous tags are valueable
    /// do not run this until they have been back up, or ported to standard tags.
    #[clap(long, conflicts_with_all = &["clean"], action)]
    pub clean_all: bool,

    /// [Default] Set field to new given values. Previous values are deleted.
    #[clap(long, conflicts_with_all = &["append", "delete"], action, default_value_t = true)]
    pub set: bool,

    /// Append flag. If set appends values leaving existing values untouched.
    #[clap(long, conflicts_with_all = &["set", "delete"], action)]
    pub append: bool,

    /// Deletes associated values for provided fields, leaving the field unset.
    #[clap(long, conflicts_with_all = &["append", "set"], action)]
    pub delete: bool,

    /// Provides a formated list of all tags associated with each matching file.
    #[clap(long, short = 'L', conflicts_with_all = &["list"], action)]
    pub list_detailed: bool,

    /// Provides a machine readable listing of matching files.
    ///
    /// Files are only newline seperated.
    #[clap(long, short = 'l', conflicts_with_all = &["list_detailed"], action)]
    pub list: bool,

    /// Performs the modification, ready for previewing, without saving/commiting the change.
    #[clap(long, action)]
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
    /// Track/Work name
    #[clap(short, long, num_args(0..))]
    pub title: Option<Vec<String>>,

    /// Recording version (e.g. remix info)
    ///
    /// The version field may be used to differentiate multiple versions of the same track
    /// title in a single collection. (e.g. remix info)
    #[clap(long = "song-version", num_args(0..))]
    pub version: Option<Vec<String>>,

    /// The collection name to which this track belongs
    #[clap(short = 'A', long, num_args(0..))]
    pub album: Option<Vec<String>>,

    /// The track number of this piece if part of a specific larger collection or album
    #[clap(short = 'n', long,num_args(0..))]
    pub tracknumber: Option<Vec<String>>,

    /// The artist generally considered responsible for the work.
    ///
    /// In popular music this is usually the performing band or singer. For classical music
    /// it would be the composer. For an audio book it would be the author of the original text.
    #[clap(short = 'a', long,num_args(0..))]
    pub artist: Option<Vec<String>>,

    /// The artist(s) who performed the work.
    ///
    /// In classical music this would be the conductor,
    /// orchestra, soloists. In an audio book it would be the actor who did the reading. In
    /// popular music this is typically the same as the ARTIST and is omitted.
    #[clap(long, num_args(0..))]
    pub performer: Option<Vec<String>>,

    /// Copyright attribution, e.g., '2001 Nobody's Band' or '1999 Jack Moffitt'
    #[clap(long, num_args(0..))]
    pub copyright: Option<Vec<String>>,

    /// License information, for example, 'All Rights Reserved'
    ///
    /// License information, for example, 'All Rights Reserved', 'Any Use Permitted', a URL to a
    /// license such as a Creative Commons license (e.g. \"creativecommons.org/licenses/by/4.0/\"),
    /// or similar.
    #[clap(long, num_args(0..))]
    pub license: Option<Vec<String>>,

    /// Name of the organization producing the track (i.e. the 'record label')
    #[clap(long, short = 'o', num_args(0..))]
    pub organization: Option<Vec<String>>,

    /// A short text description of the contents
    #[clap(long, num_args(0..))]
    pub description: Option<Vec<String>>,

    /// A short text indication of music genre
    #[clap(short, long, num_args(0..))]
    pub genre: Option<Vec<String>>,

    /// Date the track was recorded
    #[clap(short, long, num_args(0..))]
    pub date: Option<Vec<String>>,

    /// Location where track was recorded
    #[clap(long, num_args(0..))]
    pub location: Option<Vec<String>>,

    /// Contact information for the creators or distributors of the track.
    ///
    /// This could be a URL, an email address, the physical address of the producing label.
    #[clap(long, num_args(0..))]
    pub contact: Option<Vec<String>>,

    /// ISRC number for the track
    ///
    /// See the ISRC intro page for more information on ISRC numbers.
    #[clap(long, num_args(0..))]
    pub isrc: Option<Vec<String>>,
}
