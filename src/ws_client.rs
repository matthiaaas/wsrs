use std::{
    io::{BufReader, Read, Write},
    net::TcpStream,
};

use crate::frame::Frame;

pub enum WsClientError {
    FailedHandshake(String),
    FailedRecv(String),
}

pub struct WsClient<'a> {
    stream: &'a mut TcpStream,
    resource: &'a str,
    host: &'a str,
}

impl<'a> WsClient<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        WsClient {
            stream,
            resource: "/",
            host: "127.0.0.1:8765",
        }
    }

    pub fn handshake(&mut self) -> Result<String, WsClientError> {
        let sec_websocket_key = self.generate_sec_websocket_key();

        let request = format!(
            "GET {} HTTP/1.1\r\n\
            Host: {}\r\n\
            Connection: Upgrade\r\n\
            Upgrade: websocket\r\n\
            Sec-WebSocket-Key: {}\r\n\
            Sec-WebSocket-Version: 13\r\n\r\n",
            self.resource, self.host, sec_websocket_key
        );
        self.stream.write(request.as_bytes()).map_err(|e| {
            WsClientError::FailedHandshake(format!("Failed to write to stream: {}", e))
        })?;

        let mut response = String::new();
        let reader = BufReader::new(&mut self.stream);

        for byte in reader.bytes() {
            let byte = byte.map_err(|e| {
                WsClientError::FailedHandshake(format!("Failed to read from stream: {}", e))
            })?;
            response.push(byte as char);
            if response.ends_with("\r\n\r\n") {
                break;
            }
        }

        Ok(response)
    }

    pub fn send(&mut self, message: &str) {
        let frame = Frame {
            fin: true,
            rsv1: false,
            rsv2: false,
            rsv3: false,
            opcode: 0x1,
            mask: true,
            payload_length: message.len() as u64,
            masking_key: Some([0x00, 0x00, 0x00, 0x00]),
            payload_data: message.as_bytes().to_vec(),
        };

        self.stream.write_all(&frame.to_bytes()).unwrap();

        let buffer = &mut [0; 1024];
        self.stream.read(buffer).unwrap();
        println!("{:?}", buffer);
    }

    fn recv(&mut self) -> Result<(), WsClientError> {
        let reader = BufReader::new(&mut self.stream);

        for byte in reader.bytes() {
            let byte = byte.map_err(|e| {
                WsClientError::FailedRecv(format!("Failed to read from stream: {}", e))
            })?;
        }

        Ok(())
    }

    fn close(&mut self) {
        unimplemented!()
    }

    fn generate_sec_websocket_key(&mut self) -> String {
        "x3JJHMbDL1EzLkh9GBhXDw==".to_string()
    }
}
