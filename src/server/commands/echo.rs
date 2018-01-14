use server::ServerCommand;
use tags::TagDatabase;

pub struct EchoCommand;

impl EchoCommand {
    pub fn new() -> EchoCommand {
        EchoCommand {}
    }
}

impl ServerCommand for EchoCommand {
    fn can_execute(&self, command: &str) -> bool {
        return command.starts_with("echo ");
    }

    fn execute(&self, command: &str, _tag_database: &mut TagDatabase) -> String {
        let arg_separator = command.find(" ");
        let args = match arg_separator {
            None => "",
            Some(v) => &command[v + 1..]
        };
        return String::from(args);
    }
}

