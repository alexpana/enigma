use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Server {}

trait ServerCommand {
}

impl Server {
    pub fn bind(self: &mut Server, hostname: &str, port: usize) {
        let listener = TcpListener::bind(format!("{}:{}", hostname, port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(self: &mut Server, mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}