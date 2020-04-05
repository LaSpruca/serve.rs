extern crate chrono;

pub mod thread_pool;
pub mod config;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use crate::thread_pool::ThreadPool;

fn main() {
    let config = config::Config::new();
    let listener = match TcpListener::bind(format!("{}:{}", config.ip, config.port)) {
        Ok(a) => a,
        Err(_) => {println!("DUMB BITCH"); std::process::exit(69)}
    };
    let pool = match ThreadPool::new(100) {
        Ok(a) => a,
        Err(_) => {println!("FUCKING ASS HOLE"); std::process::exit(69);}
    };
    for stream in listener.incoming() {
        match stream {
            Err(_) => {println!("CONNECTION FUCKING FAILED");},
            Ok(a) => pool.execute(|| {
                handle_connection(a);
            })
        }
    }
}

fn start_server() {

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();}
        Ok(_) => {}
    };
    
    let get = b"GET / HTTP/1.1\r\n";
    let (mut response_status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 DUMB FUCKING BITCH","404.html")
    };

    let file = match fs::read_to_string(filename) {
        Err(_) => {println!("SHIT FUCK FUCK"); response_status = "HTTP/1.1 500 STUPID ASS BITCH"; String::new()}
        Ok(a) => {a}
    };

    let date = chrono::Utc::now();

    let headers = format!(
        "Content-Type: text/html\nContent-Length: {}\nDate: {}\nServer: insults\nAccept-Ranges: bytes\nVary: Vary: Accept-Encoding",
        file.len(),
        date.format("%a, %b %e %Y %T GMT+12")
    );

    let response = format!("{}\r\n{}\n\r\n{}", response_status, headers, file);

    match stream.write(response.as_bytes()) {
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();},
        Ok(_) => {}
    };
    match stream.flush(){
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();},
        Ok(_) => {}
    };
}