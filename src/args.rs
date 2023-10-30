use clap::Parser;

/// Use Brainfuck to create a server
#[derive(Parser)]
pub struct Args {
    /// The Brainfuck file to interpret.
    pub file: String,
}
