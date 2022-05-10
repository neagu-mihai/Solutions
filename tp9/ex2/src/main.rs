use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // TODO 1 - use the arguments to get an address and a port
    // cargo run address port
    let args: Vec<String> = env::args().collect();
    let address = args[1].clone().to_string();
    let port =args[2].clone().to_string();

    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(mut stream) => {
            println!("Connected to {}", stream.peer_addr().unwrap());
            stream.write("Hello".as_bytes()).unwrap();

            // TODO 2 read back from the server
            println!("Client connection {}", stream.peer_addr().unwrap());
            let mut buffer = [0; 1024]; 
           let bytes = stream.read(&mut buffer).unwrap();
                    println!("read {} bytes", bytes);
                    let s = String::from_utf8_lossy(&buffer[0..bytes]);

                    println!("Message: {}", s);
        }
        Err(error) => {
            println!("Connection failed: {}", error);
        }
    }
}
