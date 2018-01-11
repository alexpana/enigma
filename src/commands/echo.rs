pub struct EchoCommand;
use server::ServerCommand;

impl ServerCommand for EchoCommand {
    fn command_name(&self) -> String {
        return String::from("echo");
    }

    fn execute(&self, command: &str) -> String {
        let arg_separator = command.find(" ");
        let args = match arg_separator {
            None => "",
            Some(v) => &command[v + 1..]
        };
        return String::from(args);
    }
}