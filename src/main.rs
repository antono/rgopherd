use std::net::{TcpListener, TcpStream};
use std::io::prelude::{BufRead, Write};
use std::io::BufReader;
use clap::{App, Arg};

#[macro_use]
extern crate clap;

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
    stream
        .write_all(response.as_bytes())
        .expect("cannot send response");
}

fn handle_stream(stream: &mut TcpStream) {
    handle_input(stream);
    send_response(stream);
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .default_value("7070")
                .help("Sets a custom port number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .default_value("127.0.0.1")
                .help("Sets a custom host")
                .takes_value(true),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let host = matches.value_of("host").unwrap();
    let bind_to = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&bind_to).unwrap();

    println!("\n=> Listening on {}", &bind_to);

    loop {
        match listener.accept() {
            Err(_) => println!("Cannot bind to {}", &bind_to),
            Ok(mut stream) => handle_stream(&mut stream.0),
        }
    }
}
