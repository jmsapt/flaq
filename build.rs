#![allow(dead_code)]

// include!("src/cli.rs");

use clap::{CommandFactory, ValueEnum};
use std::ffi::OsString;
use std::fs;

use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

include!("src/cli.rs");
const BINARY_NAME: &str = "flaq";

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = CliArgs::command();
    for &shell in Shell::value_variants() {
        let file = generate_to(shell, &mut cmd, BINARY_NAME, &outdir)?;

        println!("cargo::warning=generate completion file at {file:?}")
    }

    Ok(())
}
