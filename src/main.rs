use std::net::{TcpListener, TcpStream};
use std::io::prelude::Read;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).expect("Error: Failed to read from client.");

    let request = String::from_utf8_lossy(&buffer[..]);

    println!("{}", request);
}

fn start_server(addr: &str) {
    let listener: TcpListener = TcpListener::bind(addr).expect("Error: Failede to bind to address");
    println!("Server listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn main() {
    start_server("127.0.0.1:3000");
}
