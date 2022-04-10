use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:3998").unwrap();
    let msg_to_send = "Hello from TCP client";
    stream.write(msg_to_send.as_bytes()).unwrap();

    let mut buffer = [0; 200];
    stream.read(&mut buffer).unwrap();

    println!("Got echo back from server: {:?}", std::str::from_utf8(&buffer)
        .unwrap()
        .trim_end_matches(char::from(0)));
}