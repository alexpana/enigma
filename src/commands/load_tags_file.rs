use server::ServerCommand;
use tags::TagDatabase;
use tags::TagDefinition;
use tags::TagFile;
use std::collections::HashMap;

struct LoadTagsFileCommand {
}

impl ServerCommand for LoadTagsFileCommand {
    fn command_name(&self) -> String {
        String::from("load-tags-file")
    }

    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();
        let file_path = tokens[1].trim();

        let tag_file = TagFile::from_file(file_path);
        // tag_database.parse_file(&tag_file);

        return String::from("Done");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_tags() {
        let mut tag_map = HashMap::new();
        tag_map.insert(String::from("tags"), vec!(TagDefinition::new_file("/classes/Test.h"),
                                                  TagDefinition::new_file("/private/Test.cpp")));
        
        let tag_database = TagDatabase {
            tags: tag_map,
        };
        
        let command = LoadTagsFileCommand::new(&mut tag_database);
        
        assert!(false);
    }
}
