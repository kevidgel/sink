//! Client implementation for Sink.
//! 

use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

const BUF_SIZE: usize = 8192;

/// Represents a SinkClient.
/// Syncs files from remote on specific address/port.
pub struct SinkClient {
    /// Address of server.
    addr: String,
    /// Target sync directory.
    target_dir: String,
    /// Receiving buffer.
    recv_buf: [u8; BUF_SIZE],
    /// Sending buffer.
    send_buf: [u8; BUF_SIZE],
    /// Command buffer.
    command: String,
}

impl SinkClient {
    /// Creates new SinkClient.
    pub fn new(addr: String, target_dir: String) -> SinkClient {
        SinkClient {
            addr,
            target_dir,
            recv_buf: [0; BUF_SIZE],
            send_buf: [0; BUF_SIZE],
            command: String::new(),
        }
    }

    /// Starts operation of SinkClient.
    pub fn start(&mut self) {
        let stream : TcpStream = self.connect_address(); 

        // TODO: Separate execution into different threads       
        loop {
            match self.prompt() {
                Ok(size) => self.send_command(&stream, size),
                Err(_error) => println!("sink: error parsing command"),
            }
            match self.read(&stream) {
                Ok(_size) => {
                    let message = match str::from_utf8(&self.recv_buf) {
                        Ok(message) => message.to_string(),
                        Err(error) => format!("received invalid utf8 sequence: {:?}", error),
                    };
                    // TODO: Parse server response
                    println!("Message: {}", message);
                },
                Err(_error) => {},
            }
            // TODO: Handle writes
            match self.write(&stream) {
                Ok(_size) => {},
                Err(_error) => {},
            }
        }
    }

    /// Send command from command buffer.
    fn send_command(&mut self, mut stream: &TcpStream, size: usize) {
        match stream.write_all(&self.command.as_bytes()[0..size]) {
            Ok(()) => {},
            Err(error) => println!("sink: error sending command: {:?}", error),
        }
    } 

    /// Write user input into command buffer.
    fn prompt(&mut self) -> io::Result<usize> {
        self.command.clear(); 
        println!("sink>");
        io::stdin().read_line(&mut self.command)
    }

    /// Read and place into receiving buffer.
    fn read(&mut self, mut stream: &TcpStream) -> io::Result<usize> {
        stream.read(&mut self.recv_buf)
    }

    /// Write from sending buffer.
    fn write(&mut self, mut stream: &TcpStream) -> io::Result<usize> {
        stream.write(&mut self.send_buf)
    }

    /// Connect SinkClient to specified address.
    fn connect_address(&mut self) -> TcpStream {
        match TcpStream::connect(&self.addr) {
            Ok(stream) => {
                println!("Connected successfully to {}", self.addr);
                stream
            },
            Err(error) => {
                panic!("connection to server failed: {:?}", error);
            }
        }
    }
}