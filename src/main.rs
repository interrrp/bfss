use clap::Parser;

use bfss::Args;

fn main() {
    let args = Args::parse();

    println!("Starting {}", args.file);
}
