use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // TODO 1 - use the arguments to get an address and a port
    // cargo run address port
    let address = "".to_string();
    let port = 0;
    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(mut stream) => {
            println!("Connected to {}", stream.peer_addr().unwrap());
            stream.write("Hello".as_bytes()).unwrap();

            // TODO 2 read back from the server
        }
        Err(error) => {
            println!("Connection failed: {}", error);
        }
    }
}
