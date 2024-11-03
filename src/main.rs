// https://datatracker.ietf.org/doc/html/rfc6455#section-5.2

use std::net::TcpStream;

use ws_client::WsClient;

mod frame;
mod ws_client;

fn main() {
    match TcpStream::connect("127.0.0.1:8765") {
        Ok(mut stream) => {
            let mut ws_client = WsClient::new(&mut stream);
            let handshake_response = ws_client.handshake().unwrap();
            println!("{}", handshake_response);
            ws_client.send("Hello world!");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
