extern crate clap;
extern crate fs_swap;

use std::path::PathBuf;

use clap::Parser;

/// Atomically swap two files
#[derive(Parser)]
struct Args {
    /// The first file to swap
    a: PathBuf,

    /// The second file to swap
    b: PathBuf,
}

fn main() {
    let args = Args::parse();
    fs_swap::swap(args.a, args.b).unwrap();
}
