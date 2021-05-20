use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() {
    let args = Opt::from_args();
    println!("{:?}", args)
}
