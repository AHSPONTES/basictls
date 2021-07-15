use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream =
        TcpStream::connect("www.google.com:80").expect("Unable to connect to the server");

    let request = String::from("GET / HTTP/1.1\r\nHost: www.google.com\r\n\r\n");

    let mut response: [u8; 4096] = [0; 4096];

    stream.write(request.as_bytes())?;
    stream.read(&mut response)?;

    println!("{}", str::from_utf8(&response).unwrap());

    Ok(())
}
