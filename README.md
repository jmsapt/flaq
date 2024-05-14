# Flaq
[![Build Status](https://github.com/jmsapt/flaq/actions/workflows/CICD.yml/badge.svg?branch=master)](https://github.com/jmsapt/flaq/actions/workflows/CICD.yml)
[![codecov](https://codecov.io/gh/jmsapt/flaq/branch/master/graph/badge.svg)](https://codecov.io/gh/jmsapt/flaq)
[![crates.io](https://img.shields.io/crates/v/flaq.svg)](https://crates.io/crates/flaq)

## Contents
- [Installation](#Installation)
- [Usage](#Usage)
- [Queries](#Queries)
- [The Standard](#The-Standard)


## Installation
### Cargo Install
The crates.io listing can be found [here](https://crates.io/crates/flaq).

Installation with `cargo` is shown below. 
```bash
cargo install flaq
```


Please not that the `cargo` install doesn't comes with the autocompletion
scripts, so shell autocompletion will not work unless you manually copy the scripts into the respective `/usr/share/`
directories.

### Arch Linux AUR
The AUR listing can be found [here](https://aur.archlinux.org/packages/flaq).

Installation with `yay` is shown below. If you are using `paru` simply substitute `paru` for `yay`.
```bash
yay -S flaq
```


## Usage
Below is the short help detailing the usage (accessible via `flaq -h`). For detailed help run `flac --help`.
```
A CLI tool for editing and query `.flac` files metadata tags

Usage: flaq [OPTIONS]

Options:
  -v, --version-number  Print version
  -c, --clean           Clean duplicated fields, where both the tag field and value match
      --clean-all       [Caution] Clean duplicated fields, and deletes ALL non-standard header comments
      --set             [Default] Set field to new given values. Previous values are deleted
      --append          Append flag. If set appends values leaving existing values untouched
      --delete          Deletes associated values for provided fields, leaving the field unset
  -L, --list-detailed   Provides a formated list of all tags associated with each matching file
  -l, --list            Provides a machine readable listing of matching files
      --dry-run         Performs the modification, ready for previewing, without saving/commiting the change
  -h, --help            Print help (see more with '--help')

Tag Fields:
  -t, --title [<TITLE>...]
          Track/Work name
      --song-version [<VERSION>...]
          Recording version (e.g. remix info)
  -A, --album [<ALBUM>...]
          The collection name to which this track belongs
  -n, --tracknumber [<TRACKNUMBER>...]
          The track number of this piece if part of a specific larger collection or album
  -a, --artist [<ARTIST>...]
          The artist generally considered responsible for the work
      --performer [<PERFORMER>...]
          The artist(s) who performed the work
      --copyright [<COPYRIGHT>...]
          Copyright attribution, e.g., '2001 Nobody's Band' or '1999 Jack Moffitt'
      --license [<LICENSE>...]
          License information, for example, 'All Rights Reserved'
  -o, --organization [<ORGANIZATION>...]
          Name of the organization producing the track (i.e. the 'record label')
      --description [<DESCRIPTION>...]
          A short text description of the contents
  -g, --genre [<GENRE>...]
          A short text indication of music genre
  -d, --date [<DATE>...]
          Date the track was recorded
      --location [<LOCATION>...]
          Location where track was recorded
      --contact [<CONTACT>...]
          Contact information for the creators or distributors of the track
      --isrc [<ISRC>...]
          ISRC number for the track

Arguments:
  -q, --query <QUERY>
  -f, --files [<FILES>...]
```

## Queries
A query is is a string of operations that is evaulated against every supplied file, excluding those files that don't
satisfy the query.

A query is a combination of variables, literals and operators. The query is an expression that can be any other combination
of expressions, but it must finally evaulate to a boolean.

### Variables
Any standard tag can be used as an variable. That variable will be replaced by the list of matching tags for each
respective file. The variables are used by their full tag name which is case-insensitive (e.g. `Title`, `tracknumber`, `ArtISt`
are all valid uses of variables).

A query will error if the a required variable on any provided file is unset.

- Date
    - The first tag for the `date` tag, that must be stored in the format `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. Note that 
    date literals for querys are prefixed by `d`, but stored dates are not
- Tracknumber
    - The first tag for the `tracknumber` tag, that must be parsable as a unsigned 32-bit integer.
    - If tracknumber is unset, or not parsable, this will error and casue the query to fail
- Other Tags
    - All other tags will be subsituted for the list of strings that are set for that respective tag

### Literals
- String Literal `"<string>"`
    - String literals must be wrapped in quoatation marks, escape characters (including for additional quoatation marks)
    are allowed. Strings can include any unicode characters
    - For example; `"Casiopea"`, `"瀬葉淳"`, `"Some \"Escape Characters\""`
- Integer Literal `<num>`
    - Integer literals must parsable as a 32-bit unsigned integer (positive integers inclusive of 0)
    - For example; `10`, `0`, `1234`
- Date Literal (`d<YYYY>`, `d<YYYY>-<MM>`, or `d<YYYY>-<MM>-<DD>`
    - Dates must be prefix by a `d` and can be given as either year, year-month, or year-month-day forms.
    - For example; `d1980`, `d2001-01`, `d1192-03-12`

### Operators
#### Comparative Operators
Compare any 2 pairs of matching types. Evaluates to a boolean expresion.
- `==` Equals
- `?=` Conatains
- `!=` Not Equals

#### Numeric Comparative Operators 
Operates on any 2 dates or integers. Evaluates to a boolean expression.
- `>` Greater Than
- `>=` Greater Than or Equals
- `<` Less Than
- `<=` Less Than or Equals

#### Logical Operators
Operates on 2 boolean expressions (or 1 for logical not). Evaulates to a boolean expression.
- `!`
- `&&`
- `||`

## The Standard
The standard, listed below, is not super rigid with room for ambiguity. This program follow this standard
completely with the following exception;
- the fields `Date` and `Tracknumber` will support any number of arguments, however the 0th tag must be match the following 
formats
  - `Date` must be given as `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. For the purpose of comparisons the more specific dates is 
  truncated to match the less specific dates (i.e. comparing `YYYY-MM-DD` and `YYYY` will truncate the first date so that the
  result of the operation is just comparing the years)
  - `Tracknumber` must be able to be parsed as an `u32` (unsigned 32-bit integer). It is up to the user to decide whether to 
  index tracks from `1` or `0` (however it suggested to index from `1` for consistency with the real track numbers).
- Only the 0th `Date`/`Tracknumber` will be used for comparison (and hence must match the above requirements). Any other
tags can optionally be included to give greater context.


[Vorbis Comment Standard (Flac Metadata Tags)](https://www.xiph.org/vorbis/doc/v-comment.html)

> ### Field names
> 
> Below is a proposed, minimal list of standard field names with a description of intended use. No single or group of field names is mandatory; a comment header may contain one, all or none of the names in this list.
> 
> - TITLE
>   - Track/Work name
> - VERSION
>   - The version field may be used to differentiate multiple versions of the same track title in a single collection. (e.g. remix info)
> - ALBUM
>   - The collection name to which this track belongs
> - TRACKNUMBER
>   - The track number of this piece if part of a specific larger collection or album
> - ARTIST
>   - The artist generally considered responsible for the work. In popular music this is usually the performing band or singer. For classical music it would be the composer. For an audio book it would be the author of the original text.
> - PERFORMER
>   - The artist(s) who performed the work. In classical music this would be the conductor, orchestra, soloists. In an audio book it would be the actor who did the reading. In popular music this is typically the same as the ARTIST and is omitted.
> - COPYRIGHT
>   - Copyright attribution, e.g., '2001 Nobody's Band' or '1999 Jack Moffitt'
> - LICENSE
>   - License information, for example, 'All Rights Reserved', 'Any Use Permitted', a URL to a license such as a Creative Commons license (e.g. "creativecommons.org/licenses/by/4.0/"), or similar.
> - ORGANIZATION
>   - Name of the organization producing the track (i.e. the 'record label')
> - DESCRIPTION
>   - A short text description of the contents
> - GENRE
>   - A short text indication of music genre
> - DATE
>   - Date the track was recorded
> - LOCATION
>   - Location where track was recorded
> - CONTACT
>   - Contact information for the creators or distributors of the track. This could be a URL, an email address, the physical address of the producing label.
> - ISRC
>   - ISRC number for the track; see the ISRC intro page for more information on ISRC numbers.
> 
> ### Implications
> Field names should not be 'internationalized'; this is a concession to simplicity not an attempt to exclude the majority of the world that doesn't speak English. Field contents, however, use the UTF-8 character encoding to allow easy representation of any language.
> We have the length of the entirety of the field and restrictions on the field name so that the field name is bounded in a known way. Thus we also have the length of the field contents.
> Individual 'vendors' may use non-standard field names within reason. The proper use of comment fields should be clear through context at this point. Abuse will be discouraged.
> There is no vendor-specific prefix to 'nonstandard' field names. Vendors should make some effort to avoid arbitrarily polluting the common namespace. We will generally collect the more useful tags here to help with standardization.
> Field names are not required to be unique (occur once) within a comment header. As an example, assume a track was recorded by three well know artists; the following is permissible, and encouraged:

