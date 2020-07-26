extern crate ezio;
use ezio::ThreadPool;
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let root = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (headers, filename) = if buffer.starts_with(root) {
        ("HTTP/1.1 200 OK\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();
    
    let response = format!("{}\r\n{}", headers, body);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}