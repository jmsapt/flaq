#![allow(dead_code)]

use std::{error::Error, fs::create_dir_all, path::PathBuf};

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};

const BINARY_NAME: &str = "flaq";
include!("src/cli.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let root_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let scripts_target = PathBuf::from(root_dir).join("target").join("scripts");

    // create directory
    create_dir_all(&scripts_target).unwrap();

    let mut cmd = CliArgs::command();
    for &shell in Shell::value_variants() {
        let _file = generate_to(shell, &mut cmd, BINARY_NAME, &scripts_target)?;
        #[cfg(debug_assertions)]
        {
            println!("cargo:warning=Autocomplete script generated at {_file:?}");
        }
    }

    Ok(())
}
