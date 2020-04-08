extern crate chrono;

pub mod thread_pool;
pub mod conf;
pub mod request;
mod errors;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use crate::thread_pool::ThreadPool;
use std::fs::read_to_string;
use crate::conf::Config;
use std::ffi::OsString;
use crate::request::Request;
use std::fs;

fn main() {
    let config = conf::Config::new();
    let listener = match TcpListener::bind(format!("{}:{}", config.ip, config.port)) {
        Ok(a) => a,
        Err(_) => {println!("DUMB BITCH"); std::process::exit(69)}
    };

    let pool = match ThreadPool::new(100) {
        Ok(a) => a,
        Err(_) => {println!("FUCKING ASS HOLE"); std::process::exit(69);}
    };
    println!("Started server on {}:{}", config.ip, config.port);
    for stream in listener.incoming() {
        match stream {
            Err(_) => {println!("CONNECTION FUCKING FAILED");},
            Ok(a) => pool.execute(move || {
                handle_connection(a, Config::new());
            })
        }
    }
}

fn handle_connection(mut stream: TcpStream, config: crate::conf::Config) {
    println!("\n");
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();}
        Ok(_) => {}
    };

    let buff = String::from_utf8_lossy(&buffer);
    let buff = String::from(buff);

    let request = crate::request::Request::new(buff);

    println!("[{}] {} requested {}", request.get_method(), stream.peer_addr().unwrap(), request.get_path());

    let cwd = std::env::current_dir().unwrap();
    let cwd2 = OsString::from(cwd);
    let path = cwd2.to_str().unwrap();

    for (k, _) in request.get_body() {
        println!("{}", k);
    };

    let requested = match request.get_value(String::from("Accept")) {
        Some(a) => a,
        None => {send_html_response("500 BITCH", errors::e_500(), &mut stream); return ()},
    };

    println!("{}", requested);

    if request.get_path() == String::from("/favicon.ico") {
        let file = fs::File::open(&format!("{}/{}/favicon.ico", path, config.resource_location));
        let metadata = fs::metadata(&format!("{}/{}/favicon.ico", path, config.resource_location));
        if file.is_ok() && metadata.is_ok(){
            let mut file = file.unwrap();
            let metadata = metadata.unwrap();
            let mut bytes = vec![0u8; metadata.len() as usize];
            match file.read(&mut bytes) {
                Err(_) => send_500(&mut stream, &config, String::from(path)),
                Ok(_) => {
                    send_image_response("200 OK", bytes, &mut stream, String::from("x-icon"));
                }
            };
        } else {
            send_404(&mut stream, &config, String::from(path));
        }
        return ();
    }

    'yes: for kind in requested.split(",") {
        println!("Kind: {}", kind);
        if kind == String::from("text/html") {
            handle_html_file(&mut stream, &config, format!("{}", path), request);
            break 'yes;
        }
    }
}

fn handle_html_file(stream: &mut TcpStream, config: &Config, path: String, request: Request) {
    if request.get_path() == format!("/") {
        println!("Getting index from {}", format!("{}/{}/index.html", path, config.resource_location));
        let file = match std::fs::read_to_string(format!("{}/{}/index.html", path, config.resource_location)) {
            Ok(o) => o,
            Err(_) => {send_404(stream, config, format!("{}", path)); return ()},
        };
        send_html_response("200 OK", file, stream);
    } else if request.get_path().ends_with(&String::from("/")) {
        let file = match std::fs::read_to_string(format!("{}/{}/{}/index.html", path, config.resource_location, request.get_path())) {
            Ok(o) => o,
            Err(_) => {send_404(stream, config, format!("{}", path)); return ()},
        };
        send_html_response("200 OK", file, stream);
    } else {
        let file = match std::fs::read_to_string(format!("{}/{}/{}", path, config.resource_location, request.get_path())) {
            Ok(o) => o,
            Err(_) => {send_404(stream, config, format!("{}", path)); return ()},
        };
    }
}

fn send_404 (stream: &mut TcpStream, config: &Config, path: String){
    if config.default_errors {
        send_html_response("404 FUCKING CUNT", errors::e_404(), stream);
    } else {
        let file = match std::fs::read_to_string(format!("{}/{}/404.html", path, config.resource_location)) {
            Ok(a) => a,
            Err(_) => {send_html_response("404 FUCKING CUNT", errors::e_404(),stream); return ()},
        };
        send_html_response("404 FUCKING CUNT", file, stream)
    }
}

fn send_500 (stream: &mut TcpStream, config: &Config, path: String){
    if config.default_errors {
        send_html_response("500 FUCKING CUNT", errors::e_500(), stream);
    } else {
        let file = match std::fs::read_to_string(format!("{}/{}/500.html", path, config.resource_location)) {
            Ok(a) => a,
            Err(_) => {send_html_response("500 FUCKING NIGGA", errors::e_404(),stream); return ()},
        };
        send_html_response("500 FUCKING NIGGA", file, stream)
    }
}

fn send_image_response (response_header: &str, image: Vec<u8>, stream: &mut TcpStream, content_type: String) {
    let date = chrono::Utc::now();

    let header =
        format!("Content-Type: {}\nContent-Length: {}\nDate: {}\nServer: \"insults 1.0 WindowsNT\"\n", content_type,
                image.len(), date.format("%a, %b %e %Y %T GMT+12"));

    let response = format!("HTTP/1.1 {}\r\n{}\r\n", response_header, header);

    match stream.write(response.as_bytes()) {
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();},
        Ok(_) => {}
    };

    match stream.write(image.as_slice()) {
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();},
        Ok(_) => {}
    };

    match stream.flush(){
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();},
        Ok(_) => {}
    };
    println!("Sent response");
}

fn send_html_response (response_header: &str, body: String, stream: &mut TcpStream) {
    let date = chrono::Utc::now();

    let header =
        format!("Content-Type: text/html\nContent-Length: {}\nDate: {}\nServer: \"insults 1.0 WindowsNT\"\n",
            body.as_bytes().len(), date.format("%a, %b %e %Y %T GMT+12"));

    let response = format!("HTTP/1.1 {}\r\n{}\r\n{}", response_header, header, body);

    match stream.write(response.as_bytes()) {
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();},
        Ok(_) => {}
    };
    match stream.flush(){
        Err(_) => {println!("FUCK FUCK FUCK FUCK"); return ();},
        Ok(_) => {}
    };
    println!("Sent response");
}
