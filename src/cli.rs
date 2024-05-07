use clap::{Args, Command, Parser};

#[derive(Parser, Debug)]
#[clap(
    // override_usage = "flaq [FLAGS] [OPTIONS] [ARGUMENTS]",
    about = "Short about - ToDO",
    long_about = "TODO",
    version,
    author,
)]
pub struct CliArgs {
    /// Clean duplicated fields, where both the tag field and value match.
    ///
    /// Runs after all edits (if used alongside edits).
    #[clap(long, short)]
    pub clean: bool,

    /// Allow other (non-standard) tags.
    #[clap(long, short)]
    pub other: bool,

    /// Opt to append tags, instead of setting them (default behaviour).
    #[clap(long)]
    pub append: bool,

    #[clap(long, short)]
    pub echo: Vec<String>,

    #[command(flatten)]
    pub opterations: Operations,

    #[command(flatten)]
    pub arguments: Arguments,
}

#[derive(Args, Debug)]
#[clap(next_help_heading = "Operations")]
pub struct Operations {
    /// Deletes all values for the provided tag.
    ///
    /// Runs first, however if preference using `--set` instead if attempting to
    /// overwrite tags.
    #[clap(long, short)]
    pub delete: bool,

    /// Sets values for each associated field to new list of given values, replacing the previous
    /// values with the new set of values.
    ///
    /// Functionally the same as deleting current values then inorder adding
    /// the provided list of new values.
    #[clap(long, short)]
    pub set: Vec<String>,
    // /// Appends the list of given values for each associated field, leaving existing values
    // /// unchanged.
    // ///
    // /// Runs after `set`.
    // #[clap(long, short)]
    // append: Vec<String>,
}

#[derive(Args, Debug)]
#[clap(next_help_heading = "Arguments")]
pub struct Arguments {
    /// List of files
    #[clap(long, short, name = "FILES")]
    pub files: Vec<std::path::PathBuf>,

    /// Query string
    #[clap(long, short, name = "QUERY")]
    pub query: Vec<String>,
}
