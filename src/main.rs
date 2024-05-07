#![allow(unused)]
use std::error::Error;

use clap::Parser;
use cli::CliArgs;
use metaflac::Tag;

mod cli;
mod operations;
mod parsers;
mod tags;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    println!("Deafult flag: {:?}", args.clean);
    // if args.foo {
    //     println!("foo")
    // }
    // if args.bar {
    //     println!("bar")
    // }:w
    // for f in args.files {
    //     let tag = Tag::read_from_path(f).unwrap();
    //     clean_fields(tag);
    // }

    Ok(())
}

fn clean_fields(tag: Tag) {
    let artists = tag.get_vorbis("ARTIST").unwrap();
    let titles = tag.get_vorbis("TITLE").unwrap();
    let x = 10;

    artists.into_iter().for_each(|a| println!("-- Artist: {a}"));
    titles.into_iter().for_each(|a| println!("-- Titles: {a}"));
}
