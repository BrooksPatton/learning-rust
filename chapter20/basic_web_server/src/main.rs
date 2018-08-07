use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let index_html = fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", index_html);
    stream.write(response.as_bytes()).unwrap();

    println!("The incomming stream:\n\n{}", String::from_utf8_lossy(&buffer[..]));
}