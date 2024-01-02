//! Client implementation for Sink.
//! 
use std::io::{Read, Write};
use std::net::TcpStream;

const BUF_SIZE: usize = 8192;

pub struct SinkClient {
    addr: String,
    target_dir: String,
    recv_buf: [u8; BUF_SIZE],
    send_buf: [u8; BUF_SIZE],
}

impl SinkClient {
    pub fn new(addr: String, target_dir: String) -> SinkClient {
        SinkClient {
            addr,
            target_dir,
            recv_buf: [0; BUF_SIZE],
            send_buf: [0; BUF_SIZE],
        }
    }

    pub fn start(&mut self) {
        let mut stream : TcpStream = self.connect_address();

        let bytes_sent = match stream.write(b"Hellooo\n") {
            Ok(bytes_sent) => bytes_sent,
            Err(error) => panic!("write error: {:?}", error),
        };

        println!("Bytes sent: {}", bytes_sent);
    }

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