use server::*;

use tags::TagDatabase;
use tags::TagDefinition;
use tags::TagLocation;
use std::path::Path;

pub struct EchoCommand;

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

pub struct DescribeTag<'a> {
    pub tag_database: &'a TagDatabase<'a>
}

impl<'a> ServerCommand for DescribeTag<'a> {
    fn command_name(&self) -> String {
        String::from("describe")
    }

    fn execute(&self, command: &str) -> String {
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

pub struct FindOtherFileCommand<'a> {
    pub tag_database: &'a TagDatabase<'a>
}

impl<'a> ServerCommand for FindOtherFileCommand<'a> {
    fn command_name(&self) -> String {
        String::from("find-other-file")
    }

    fn execute(&self, command: &str) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();

        let file_path = tokens[1].trim();
        let arg_path = Path::new(file_path);

        String::from("")
//
//        for tag in &self.tag_database.tags {
//            if tag.name.starts_with(tag_name) {
//                return format!("{:?}", tag);
//            }
//        }
//
//        String::from(format!("Tag {} not found.", tag_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tags::TagKind;

    #[test]
    fn test_find_other_file() {
        let tag_database = TagDatabase {
            tags: vec!(TagDefinition::new_file("/tmp/Test.h"),
                       TagDefinition::new_file("/tmp/Test.cpp"))
        };

        let command = FindOtherFileCommand {
            tag_database: &tag_database
        };

    }
}