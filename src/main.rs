use std::{fs, io, net::TcpListener};

use brainfuck::{program::Program, tape::ArrayTape, Interpreter};
use clap::Parser;

use bfss::{Args, Header};

fn main() -> io::Result<()> {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file)?;
    let header = Header::from_brainfuck(&contents);

    println!("Serving {} on http://{}", args.file, header.address);

    let listener = TcpListener::bind(header.address)?;
    for stream in listener.incoming() {
        // Since `Program` unfortunately does not derive from Clone, we have to construct one on
        // every connection

        let mut stream = stream?;
        let mut stream_clone = stream.try_clone()?;

        let program = Program::parse(&contents).unwrap();

        let mut interp = Interpreter::<ArrayTape>::new(program, &mut stream, &mut stream_clone);
        interp.run().unwrap();
    }

    Ok(())
}
