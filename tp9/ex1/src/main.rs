use std::io::Read;
use std::net::TcpListener;
use std::io::Write;

fn main() {
    // Listen for connections on 127.0.0.1 (localhost) port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Use \"nc 127.0.0.1 7878\" to send data to the server");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // a new client has connected
                println!("Client connection {}", stream.peer_addr().unwrap());
                let mut buffer = [0; 1024];

                loop {
                    let bytes = stream.read(&mut buffer).unwrap();
                    println!("read {} bytes", bytes);
                    let s = String::from_utf8_lossy(&buffer[0..bytes]);

                    println!("Message: {}", s);

                    // TODO 1 send the string back
                    stream.write(s.as_bytes()).unwrap();
                    // TODO 2 close the connection when the received string is "exit"
                    if s=="exit"{
                        stream.flush().unwrap();
                    }
                }
            }
            Err(error) => {
                println!("Error {}", error);
            }
        }
    }
}
