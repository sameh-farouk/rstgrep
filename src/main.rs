use std::path::PathBuf;
use std::io::BufReader;
use structopt::StructOpt;
use std::io::prelude::*;
use std::fs::File;
use anyhow::{Context, Result};
use regex::Regex;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
#[structopt(name = "rstGREP", about = "an alternative to grep written in Rust.")]
struct Opt {
    /// The pattern to look for
    pattern: Regex,
    /// The path to the file to read
    #[structopt(parse(from_os_str), default_value =".")]
    path: PathBuf,
    /// Show relative line number in the file
    #[structopt(short, long = "--line-number")]
    n: bool,
    /// Read all files under each directory, recursively
    #[structopt(short, long="--recursive")]
    r: bool,
    /// print only a count of selected lines per FILE
    #[structopt(short, long = "--count")]
    c: bool,
    /// Suppress normal output, instead print the name of each input file from which output would normally have been printed.
    #[structopt(short, long = "--files-with-matches", requires("r"))]
    l: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    // println!{"{:?}", &f}
    if args.r 
    {
        walk_fs(&args, &args.path);
    } else {
        let f = File::open(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
        match_pattern_in_file(&f, &args, &args.path);
    }
    //.expect(&format!("Could not read file {:?}", args.path)[..]);
    Ok(())
}

fn match_pattern_in_file(f: &std::fs::File, args: &Opt, path: &std::path::PathBuf) {
    let reader = BufReader::new(f);
    let mut count: u32 = 0;
    for (ln, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if args.pattern.is_match(&line) {
                    count = count + 1;
                    if count == 1 && args.r {
                        println!();
                        println!("-------- File Path: {} --------", path.display());
                    }
                    if !args.c && !args.l {
                        if args.n {
                            print!("{}:\t", ln+1);
                        }
                        println!("{}", line.trim());
                    }
                    
                }
            },
            Err(e) => { // mostly invalid utf8 content
                if args.r {
                    return
                } else {
                    panic!("Error! {}", e);
                }
            },
        }
    }
    if args.c && (count > 0 || !args.r){
        println!("Count: {}", count);
    }
}

fn walk_fs(args: &Opt, path: &std::path::PathBuf) {
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let f = File::open(&entry.path()).expect(&format!("Could not read file {:?}", entry.path())[..]);
                    match_pattern_in_file(&f, &args, &entry.path());
                } else if file_type.is_dir() {
                    walk_fs(&args, &entry.path());
                }
            }
        }
    }
}