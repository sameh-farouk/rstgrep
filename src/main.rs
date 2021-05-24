use std::path::PathBuf;
use std::io::BufReader;
use structopt::StructOpt;
use std::io::prelude::*;
use std::fs::File;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
#[structopt(name = "rstGREP", about = "an alternative to grep written in Rust.")]
struct Opt {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(short, parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let f = File::open(&args.path)
    .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    //.expect(&format!("Could not read file {:?}", args.path)[..]);
    let reader = BufReader::new(f);
    for (ln, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        if line.contains(&args.pattern) {
            println!("{}: {}", ln, line);
        }
    }
    Ok(())
}
