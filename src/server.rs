//! Server implementation for Sink.

use std::io::Read;
use std::net::{TcpListener, TcpStream};

const BUF_SIZE: usize = 8192;

/// Represents a SinkServer.
/// Serves a directory on a specific address/port.
pub struct SinkServer {
    /// Address the SinkServer binds to.
    binding_addr: String,
    /// Directory to serve/sync.
    target_dir: String,
    /// Receiving buffer for incoming data.
    recv_buf: [u8; BUF_SIZE],
    /// Sending buffer for outgoing data.
    send_buf: [u8; BUF_SIZE],
}

impl SinkServer {
    /// Creates new SinkServer.
    pub fn new(binding_addr: String, target_dir: String) -> SinkServer {
        SinkServer {
            binding_addr,
            target_dir,
            recv_buf: [0; BUF_SIZE],
            send_buf: [0; BUF_SIZE],
        }
    }

    /// Starts operation of SinkServer.
    pub fn start(&mut self) {
        let listener = self.bind_address();

        loop {
            self.accept_incoming(&listener);
        }
    }

    // Binds SinkServer to specified address/port.
    fn bind_address(&mut self) -> TcpListener {
        match TcpListener::bind(&self.binding_addr) {
            Ok(listener) => {
                println!("Listening on {}", self.binding_addr);
                listener
            }
            Err(error) => panic!("binding failed: {:?}", error),
        }
    }

    /// Accepts incoming TCP connections. 
    fn accept_incoming(&mut self, listener: &TcpListener) {
        for result in listener.incoming() {
            match result {
                Ok(stream) => {
                    self.handle_client(stream);
                }
                Err(error) => {
                    println!("client connection failed: {:?}", error);
                }
            }
        }
    }

    /// Handles client
    fn handle_client(&mut self, mut stream: TcpStream) {
        let bytes_read = match stream.read(&mut self.recv_buf) {
            Ok(bytes_read) => bytes_read,
            _ => 0,
        };
    
        println!("{}", bytes_read);
    }

}