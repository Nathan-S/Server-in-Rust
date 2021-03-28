use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::{Read, Error};
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Error> {
    print!("Enter IP address: ");
    let _=stdout().flush();
    let mut ipin = String::new();
    stdin().read_line(&mut ipin).expect("error:unable to read input");
    let split = ipin.split(".");
    let mut vec: Vec<&str> = split.collect();
    vec[3] = vec[3].trim();
    let mut vec1: Vec<u8> = Vec::new();
    println!("{:?}", vec);
    for v in vec {
        let a: u8 = v.parse().unwrap();
        vec1.push(a);
    }

    print!("Enter Port number: ");
    let _=stdout().flush();
    let port: u16;
    let mut input = String::new();
    stdin().read_line(&mut input).expect("error:unable to read input");
    port = input.trim().parse::<u16>().unwrap();

    let ip_add = Ipv4Addr::new(vec1[0], vec1[1], vec1[2], vec1[3]);
    let sock = SocketAddrV4::new(ip_add, port);
    let listener = TcpListener::bind(sock).unwrap();

    for connections in listener.incoming() {
        let mut connection = connections.unwrap();

        let mut input = String::new();
        let _ = connection.read_to_string(&mut input)?;
        println!("Client says {}", input);
    }
    Ok(())
}