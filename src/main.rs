pub mod lexer;
pub mod http_entity;
pub mod parser;

use std::{
    io::Read,
    iter::Peekable,
    net::{TcpListener, TcpStream}, result,
};

use lexer::TokenKind;

use crate::parser::Parser;

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

    let mut parser = Parser::new(peekable);
    let request = parser.request_line();

    println!("request {:#?}", request);

    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
