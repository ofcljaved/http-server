use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};

const LOCALHOST: &str = "127.0.0.1";
const PRIMARY_PORT: u16 = 6969;
const READ_BUFFER_SIZE: usize = 1024;

fn parse_request(buffer: &[u8]) {
    let request = String::from_utf8_lossy(buffer);
    let req_iter = request.split("\r\n\r\n");
    for x in req_iter {
        println!("{}", x);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0u8; READ_BUFFER_SIZE];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection is closed by client");
                break;
            }
            Ok(size) => {
                let buf = &buffer[..size];
                parse_request(buf);
                let response = "HTTP/1.1 200 OK\r\n\
                Content-Length: 2\r\n\
                Content-Type: text/plain\r\n\
                Connection: close\r\n\r\n\
                ok";
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(err) => {
                println!("Error reading from stream: {}", err);
                break;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut port = PRIMARY_PORT;

    let listner = loop {
        let addr = format!("{}:{}", LOCALHOST, port);
        match TcpListener::bind(&addr) {
            Ok(listner) => {
                println!("Successfully connected to {}", addr);
                break listner;
            }
            Err(err) => {
                println!("Failed to connect to {} -> \r\nError:{}", addr, err);
                port += 1;
            }
        };
    };

    for connection in listner.incoming() {
        match connection {
            Ok(str) => handle_stream(str),
            Err(err) => println!("Error in stream loop: {}", err),
        }
    }
    Ok(())
}
