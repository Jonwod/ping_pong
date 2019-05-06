use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;


fn pong(stream: &mut TcpStream) {
    let mut my_buff_buff : &mut [u8] = &mut[0, 0];
    stream.read(my_buff_buff).unwrap();
   
    let s = str::from_utf8(my_buff_buff);

    println!("Got {}", s.unwrap());
}


fn main() -> std::io::Result<()>{

    let listener = TcpListener::bind("127.0.0.1:5780")?;

    let mut stream = TcpStream::connect("51.7.187.57:5780")?;

    stream.write(&[b'h', b'a'])?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        pong(&mut stream?);
    }

    Ok(())
}
