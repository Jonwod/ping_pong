use std::io::prelude::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::str;


fn pong(stream: &mut TcpStream) {
    let mut buff = &mut [0 as u8];

    while stream.read(buff).unwrap() != 0 {
        print!("{}", str::from_utf8(buff).unwrap());
        std::io::stdout().flush().unwrap();
    }
}


fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:5780")?;

    let mut stream = TcpStream::connect("127.0.0.1:5780")?;

    let my_str : &str = "Hello there";

    stream.write(&my_str.as_bytes())?;

    for stream in listener.incoming() {
        pong(&mut stream?);
    }

    Ok(())
}
