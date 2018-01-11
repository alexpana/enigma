use server::*;

use tags::TagDatabase;
use tags::TagDefinition;
use tags::TagLocation;
use std::collections::HashMap;
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
    pub tag_database: &'a TagDatabase<'a>,
    extensions: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> FindOtherFileCommand<'a> {
    pub fn new(tag_database: &'a TagDatabase) -> FindOtherFileCommand<'a> {
        FindOtherFileCommand {
            tag_database,
            extensions: [
                ("cpp", vec!["h", "hpp"]),
                ("c", vec!["h", "hpp"]),
                ("h", vec!["c", "cpp"]),
                ("hpp", vec!["c", "cpp"]),
            ].iter().cloned().collect(),
        }
    }

    pub fn match_other_file(other_file_name: &str, file_stem: &str, extensions: &Vec<&str>) -> bool {
        return false;
    }
}

impl<'a> ServerCommand for FindOtherFileCommand<'a> {
    fn command_name(&self) -> String {
        String::from("find-other-file")
    }

    fn execute(&self, command: &str) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();

        let file_path = tokens[1].trim();
        let arg_path = Path::new(file_path);

        let file_extension_opt = arg_path.extension();
        let file_extension = match file_extension_opt {
            None => return String::from(""),
            Some(v) => v.to_str().unwrap(),
        };

        let file_name = arg_path.file_stem().unwrap().to_str().unwrap();
        let other_file_extensions = self.extensions.get(file_extension).unwrap();

        for tag in &self.tag_database.tags {
            if FindOtherFileCommand::match_other_file(tag.name, file_name, other_file_extensions) {
                return format!("{}", tag.location.file_path);
            }
        }

        String::from(format!("Other file not found."))
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

        let command = FindOtherFileCommand::new(&tag_database);

        assert_eq!("/tmp/Test.cpp", command.execute("find-other-file Test.h"));
        assert_eq!("/tmp/Test.h", command.execute("find-other-file Test.cpp"));
    }
}