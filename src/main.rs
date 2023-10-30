use std::{
    fs,
    io::{self, Write},
    net::TcpListener,
};

use clap::Parser;

use bfss::{Args, Header};

fn main() -> io::Result<()> {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file)?;
    let header = Header::from_brainfuck(&contents);

    let listener = TcpListener::bind(header.address)?;
    for stream in listener.incoming() {
        stream?.write_all(b"OK")?;
    }

    Ok(())
}
