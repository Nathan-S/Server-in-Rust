use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{stdout, stdin};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:2021")?;//change IP and port here
    let mut s = String::new();
    print!("Enter message to send:");
    let _=stdout().flush();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim();
    stream.write(t.as_bytes()).unwrap();
    Ok(())
} // the stream is closed here