#![allow(dead_code)]
const BINARY_NAME: &str = "flaq";
include!("src/cli.rs");

fn main() -> Result<(), std::io::Error> {
    use clap::{CommandFactory, ValueEnum};
    use clap_complete::{generate_to, Shell};
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = CliArgs::command();
    for &shell in Shell::value_variants() {
        let file = generate_to(shell, &mut cmd, BINARY_NAME, &outdir)?;

        // source bash autocomplete for testing
        #[cfg(debug_assertions)]
        {
            if shell == Shell::Bash {
                println!("source {file:?}");
                println!("cargo::warning=File {file:?} was source to shell (sources autocompletes for testing)");
            }
        }
    }

    Ok(())
}
