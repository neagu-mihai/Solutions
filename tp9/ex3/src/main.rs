use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;

/// The main goal is to build a simple chat server. The server
/// has the following architecture:
///    - The main thread (main) that accepts incoming connections
///      using TcpListener
///    - a thread for each incoming client that reads and writes data
///      from the TcpStream (client) - really possible?
///    - A thread (broadcaster) that distributes messages to to all the clients
///
/// The server uses the following channels:
///    client<Action> ---> server (server channel)
///         registers
///         sends a message to breadcast
///         unregisters
///    server<String> ---> client (client channels)
///         sends a message from the server to the client
///
/// Each client thread will be identified by a unique number

/// Actions that are sent
enum Action {
    /// Register a client with the server
    /// Each client is identified by a
    /// numeric id
    Register(usize, Sender<String>),

    /// Send a message for all clients
    Message(String),

    /// Unregister a client
    Unregister(usize),
}

fn main() {
    // TODO 1 - create a channel that can send an Action

    // TODO 2 - create the server thread that receives Action(s)
        // TODO 3 - create a Vec of client channels
        let mut channels = vec![];

        // TODO 4 - createv a loop where you receives Action
        //    - Register - add the (id, Sender<Action>) to the Vec
        //    - Message - iterate the channels and send the message to
        //                each channel
        //    - Unregister - delete the channel from the channels

    // TODO 6 - Listen for connections on 127.0.0.1 (localhost) port 7878
    // TODO 7 - For for each connection:
    //    - create a client channel for sending String
    //    - register the sending end of the channel with the server
    //    - start a new thread and listen for data from the clients
    //    - listen for data incoming from the server, the receiving end of
    //      the client channel and send it to the socket
    // Hint - you will have to split the stream into rend and write
    // let read_write_stream = Arc::new(stream);
    // let read_stream = read_write_stream.clone();
    // (&(*read_stream)).read(&mut buffer)
    // let write_stream = read_write_stream.clone();
    // (&(*write_stream)).write(s.as_bytes())

    // TODO 8 - use a BufReader to read lines instead of bytes
    // - the first line read is the nickname and send it
    //   to all the clients when sending a message
    // - all the other lines are messages
}
