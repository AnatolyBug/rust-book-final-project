use std::io::prelude::*; //need certain traits
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;
use final_project::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    //for loop produces a series of streams
    for stream in listener.incoming() {
        // a single stream repr an open connection
        // a connection is full (client request -> server response -> server close conn) process
        let _stream = stream.unwrap();
        pool.execute(|| {
        println!("Connection established!");
        handle_connection(_stream);
        //connection is closed as part of drop when stream goes out of scope
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024]; //on stack
    // 1024 bytes is enough for basic requests. Arbitrary size is more complicated
    stream.read(&mut buf).unwrap(); //here read changes internal state so need mut. puting stream into buf
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")               
    } else if buf.starts_with(sleep) {
        thread::sleep(Duration::from_secs(20));
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let cont = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        cont.len(),
        cont
    );
    stream.write(response.as_bytes()).unwrap();
    //flush waits and prevents program form running until all bytes are written
    stream.flush().unwrap();
    
}