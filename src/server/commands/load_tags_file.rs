use server::ServerCommand;
use tags::TagDatabase;
use tags::TagFile;

pub struct LoadTagsFileCommand {}

impl LoadTagsFileCommand {
    pub fn new() -> LoadTagsFileCommand {
        LoadTagsFileCommand {}
    }
}

impl ServerCommand for LoadTagsFileCommand {
    fn can_execute(&self, command: &str) -> bool {
        command.starts_with("load-tags-file ")
    }

    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();
        let file_path = tokens[1].trim();

        let tag_file = TagFile::from_file(file_path);
        tag_database.tag_files.push(tag_file);

        return String::from("Done");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tags::TagDefinition;
    use std::collections::HashMap;

    #[test]
    fn should_load_tags() {
    //     let mut tag_map = HashMap::new();
    //     tag_map.insert(String::from("tags"), vec!(TagDefinition::new_file("/classes/Test.h"),
    //                                               TagDefinition::new_file("/private/Test.cpp")));
        
    //     let tag_database = TagDatabase {
    //         tags: tag_map,
    //     };
        
    //     let command = LoadTagsFileCommand::new(&mut tag_database);
        
    //     assert!(false);
    }
}
