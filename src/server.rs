use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use tags::TagDatabase;

pub struct Server<'a> {
    commands: Vec<Box<ServerCommand + 'a>>,
    tag_database: &'a mut TagDatabase<'a>
}

pub trait ServerCommand {
    fn command_name(&self) -> String;
    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String;
}

impl<'a> Server<'a> {
    pub fn new(tag_database: &'a mut TagDatabase<'a>) -> Server<'a> {
        Server {
            tag_database,
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

            self.handle_connection(stream);
        }
    }

    fn handle_connection(self: &mut Server<'a>, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let request_raw_string = String::from_utf8(Vec::from(&buffer[..])).unwrap();
        let request = request_raw_string.lines().nth(0).unwrap();
        println!("Received request: \"{}\"", request);
        for command in &self.commands {
            if request.starts_with(&command.command_name()) {
                let reply = command.execute(&request, &mut self.tag_database);

                println!("Sending reply: \"{}\"", reply);
                stream.write(reply.as_bytes()).unwrap();
            }
        }
        stream.flush().unwrap();
    }
}
