extern crate basic_web_server;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use basic_web_server::ThreadPool;
use std::thread;

fn main() {
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address).unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
    }

    println!("shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let file_name;
    let mut status = 200;
    let mut status_text = "ok";
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(path("/").as_bytes()) {
        file_name = "index.html";
    }
    else if buffer.starts_with(path("/sleep").as_bytes()) {
        thread::sleep(Duration::from_secs(10));
        file_name = "sleep.html"
    } else {
        status = 404;
        status_text = "not found";
        file_name = "404.html";
    }

    let html = fs::read_to_string(file_name).unwrap();
    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", status, status_text, html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("request:\n{}", String::from_utf8_lossy(&buffer));
}

fn path(location: &str) -> String {
    format!("GET {} HTTP/1.1\r\n", location)
}