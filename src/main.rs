use std::fs;

use clap::Parser;

use bfss::{Args, Header};

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file).unwrap();

    let header = Header::from_brainfuck(&contents);

    println!("Serving {} on {}", args.file, header.address);
}
