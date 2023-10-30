use std::io::{Read, Write};

use tokio::{fs, io, net::TcpListener};

use brainfuck::{program::Program, tape::ArrayTape, Error, Interpreter};
use clap::Parser;

use bfss::{Args, Header};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file).await?;
    let header = Header::from_brainfuck(&contents);

    println!("Serving {} on http://{}", args.file, header.address);

    let listener = TcpListener::bind(header.address).await?;
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let mut stream = stream.into_std().unwrap();
        let mut stream_clone = stream.try_clone().unwrap();
        let contents = contents.clone();
        tokio::spawn(async move {
            eval_brainfuck(&contents, &mut stream, &mut stream_clone).unwrap();
        });
    }
}

fn eval_brainfuck(code: &str, input: &mut impl Read, output: &mut impl Write) -> Result<(), Error> {
    let program = Program::parse(code).unwrap();
    let mut interp = Interpreter::<ArrayTape>::new(program, input, output);
    interp.run()
}
