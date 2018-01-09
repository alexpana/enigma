use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Server<'a> {
    commands: Vec<Box<ServerCommand + 'a>>,
}

pub trait ServerCommand {
    fn command_name(&self) -> String;
    fn execute(&self, command: &String) -> String;
}

impl <'a> Server<'a> {
    pub fn new() -> Server<'a> {
        Server {
            commands: Vec::new()
        }
    }

    pub fn add_command(self: &mut Server<'a>, command: Box<ServerCommand + 'a>) {
        self.commands.push(command);
    }

    pub fn bind(self: &mut Server<'a>, hostname: &str, port: usize) {
        let listener = TcpListener::bind(format!("{}:{}", hostname, port)).unwrap();

        println!("Listening for TCP connections on {}:{}", hostname, port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(self: &mut Server<'a>, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let message = String::from_utf8(Vec::from(&buffer[..])).unwrap();

        for command in &self.commands {
            if message.starts_with(&command.command_name()) {
                stream.write(command.execute(&message).as_bytes()).unwrap();
            }
        }
        stream.flush().unwrap();
    }
}