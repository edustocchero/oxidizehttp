pub mod lexer;

use std::{
    io::Read,
    iter::Peekable,
    net::{TcpListener, TcpStream}, result,
};

use lexer::{Lexer, TokenKind};

const BUFFER_MAX_SIZE: usize = 65535;

type Result = result::Result<(), std::io::Error>;

fn main() -> Result {
    println!("Hello, world!");
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        handle(stream.unwrap())?;
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> Result {
    use std::net::Shutdown;

    let mut buffer = [0u8; BUFFER_MAX_SIZE];
    let size = stream.read(&mut buffer)?;

    let peekable: &mut Peekable<std::slice::Iter<'_, u8>> = &mut buffer[..size].iter().peekable();

    let lex = Lexer::new(peekable);
    let t = lex.collect::<Vec<TokenKind>>();
    println!("T {:?}", t);

    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
