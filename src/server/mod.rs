extern crate yaml_rust;

pub mod commands;

use std::io::prelude::*;
use std::fs::File;
use std::net::TcpListener;
use std::net::TcpStream;

use self::yaml_rust::{YamlLoader};

use tags::TagDatabase;

use self::commands::*;

pub struct Server<'a> {
    commands: Vec<Box<ServerCommand + 'a>>,
    tag_database: TagDatabase,
    command_index: usize,
}

pub trait ServerCommand {
    fn can_execute(&self, command: &str) -> bool;
    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String;
}

impl <'a> Server<'a> {
    pub fn new() -> Server<'a> {
        Server {
            tag_database: TagDatabase::new(),
            commands: Vec::new(),
            command_index: 0,
        }
    }

    pub fn start_with_config_file(config_file: &str) {
        let mut file = File::open(config_file).unwrap();
        let mut config_contents = String::new();
        file.read_to_string(&mut config_contents).unwrap();
        let docs = YamlLoader::load_from_str(&config_contents.as_str()).unwrap();

        let doc = &docs[0];

        // config
        let hostname = doc["config"]["hostname"].as_str().or_else(|| Some("localhost")).unwrap();
        let port = doc["config"]["port"].as_i64().or_else(|| Some(9092)).unwrap() as usize;

        let mut server = Server::new();
        server.add_command(Box::new(Echo::new()));
        server.add_command(Box::new(FindOtherFile::new()));
        server.add_command(Box::new(LoadTagsFile::new()));
        server.add_command(Box::new(DescribeTagCommand::new()));

        // commands
        for command in doc["commands"].as_vec().unwrap() {
            server.execute(command.as_str().unwrap());
        }

        server.bind(hostname, port);
    }
    
    pub fn add_command(&mut self, command: Box<ServerCommand>) {
        self.commands.push(command);
    }

    pub fn bind(&mut self, hostname: &str, port: usize) {
        let listener = TcpListener::bind(format!("{}:{}", hostname, port)).unwrap();

        info!("Listening for TCP connections on {}:{}", hostname, port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    pub fn execute(&mut self, request: &str) -> String {
        let command_index = self.command_index;
        self.command_index += 1;
        
        info!("Executing command [{}]: \"{}\"", command_index, request);
        let mut handled = false;
        let mut result = String::new();
        
        for command in &self.commands {
            if command.can_execute(request) {
                let reply = command.execute(&request, &mut self.tag_database);
                result = reply;
                handled = true;
                break;
            }
        }

        if !handled {
            result = "Unrecognized command".to_string();
        }

        info!("Command result [{}]: \"{}\"", command_index, result);
        result
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let request_raw_string = String::from_utf8(Vec::from(&buffer[..])).unwrap();
        let request = request_raw_string.lines().nth(0).unwrap();
        let reply = self.execute(request);

        stream.write(reply.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
