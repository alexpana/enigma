use server::*;

use tags::TagDatabase;

pub struct EchoCommand;

impl ServerCommand for EchoCommand {
    fn command_name(&self) -> String {
        return String::from("echo");
    }

    fn execute(&self, command: &String) -> String {
        let arg_separator = command.find(" ");
        let args = match arg_separator {
            None => "",
            Some(v) => &command.as_str()[v + 1..]
        };
        return String::from(args);
    }
}

pub struct DescribeTag<'a> {
    pub tag_database: &'a TagDatabase<'a>
}

impl<'a> ServerCommand for DescribeTag<'a> {
    fn command_name(&self) -> String {
        String::from("describe")
    }

    fn execute(&self, command: &String) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();

        let tag_name = tokens[1].trim();

        for tag in &self.tag_database.tags {
            if tag.name.starts_with(tag_name) {
                return format!("{:?}", tag);
            }
        }

        String::from(format!("Tag {} not found.", tag_name))
    }
}