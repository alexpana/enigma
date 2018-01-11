use server::*;

use tags::TagDatabase;
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

        for tag in self.tag_database.tags.values().flat_map(|v| v) {
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
        let path = Path::new(other_file_name);

        let file_extension_opt = path.extension();
        let extension_matches = match file_extension_opt {
            None => false,
            Some(v) => extensions.contains(&v.to_str().unwrap()),
        };

        let file_stem_opt = path.file_stem();
        let stem_matches = match file_stem_opt {
            None => false,
            Some(v) => v == file_stem,
        };

        return stem_matches && extension_matches;
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

        for tag in self.tag_database.tags.values().flat_map(|v| v) {
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
    use tags::*;

    #[test]
    fn should_find_other_file_in_different_paths() {
        let mut tag_map = HashMap::new();
        tag_map.insert(String::from("tags"), vec!(TagDefinition::new_file("/classes/Test.h"),
                                    TagDefinition::new_file("/private/Test.cpp")));

        let tag_database = TagDatabase {
            tags: tag_map,
        };

        let command = FindOtherFileCommand::new(&tag_database);

        assert_eq!("/private/Test.cpp", command.execute("find-other-file Test.h"));
        assert_eq!("/classes/Test.h", command.execute("find-other-file Test.cpp"));
    }

    #[test]
    fn should_find_other_file_in_multiple_tags() {
        let mut tag_map = HashMap::new();
        tag_map.insert(String::from("tags"), vec!(TagDefinition::new_file("/1/2/Test.h"),
                                    TagDefinition::new_file("/a/b/TestA.h"),
                                    TagDefinition::new_file("/x/y/Test.cpp"),
                                    TagDefinition::new_file("/ma/sogetsu/TestA.cpp"),
        ));

        let tag_database = TagDatabase {
            tags: tag_map
        };

        let command = FindOtherFileCommand::new(&tag_database);

        assert_eq!("/ma/sogetsu/TestA.cpp", command.execute("find-other-file TestA.h"));
        assert_eq!("/1/2/Test.h", command.execute("find-other-file /x/y/Test.cpp"));
    }
}