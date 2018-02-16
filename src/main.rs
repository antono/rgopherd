use std::net::{TcpListener, TcpStream};
use std::io::prelude::{BufRead, Write};
use std::io::BufReader;

fn handle_input(stream: &mut TcpStream) {
    let mut buf_stream = BufReader::new(stream);
    let mut input = String::new();

    while input.len() < 10 {
        let result = buf_stream.read_line(&mut input);

        match result {
            Ok(n) => println!("Received {} bytes", n),
            _ => {}
        }
    }

    println!("GOT: {}", input);
}

fn send_response(stream: &mut TcpStream) {
    let response = "\nHello world!\n\n";
    stream.write_all(response.as_bytes()).expect("cannot send response");
}

fn handle_stream(stream: &mut TcpStream) {
    handle_input(stream);
    send_response(stream);
}

fn main() {
    let bind_to = "127.0.0.1:7070";
    let listener = TcpListener::bind(bind_to).unwrap();

    println!("Listening on {}", bind_to);

    loop {
        match listener.accept() {
            Err(_) => println!("Cannot bind to {}", bind_to),
            Ok(mut stream) => handle_stream(&mut stream.0),
        }
    }
}
