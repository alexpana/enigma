pub struct EchoCommand;
use server::ServerCommand;
use tags::TagDatabase;

impl ServerCommand for EchoCommand {
    fn command_name(&self) -> String {
        return String::from("echo");
    }

    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String {
        let arg_separator = command.find(" ");
        let args = match arg_separator {
            None => "",
            Some(v) => &command[v + 1..]
        };
        return String::from(args);
    }
}
