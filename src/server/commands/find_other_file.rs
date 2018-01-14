use server::ServerCommand;

use tags::TagDatabase;
use std::collections::HashMap;
use std::path::Path;

pub struct FindOtherFileCommand {
    extensions: HashMap<&'static str, Vec<&'static str>>,
}

impl FindOtherFileCommand {
    pub fn new() -> FindOtherFileCommand {
        FindOtherFileCommand {
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

impl ServerCommand for FindOtherFileCommand {
    fn can_execute(&self, command: &str) -> bool {
        return command.starts_with("find-other-file ");
    }

    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String {
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

        for tag in tag_database.tag_files.iter().map(|v| &v.tags).flat_map(|v| v) {
            if FindOtherFileCommand::match_other_file(tag.name(), file_name, other_file_extensions) {
                return format!("{}", tag.source_file());
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
        let mut tag_file = TagFile::new();
        tag_file.tags.push(TagDefinition::from_string(file_tag_declaration("/classes/Test.h")));
        tag_file.tags.push(TagDefinition::from_string(file_tag_declaration("/private/Test.cpp")));
        
        let mut tag_database = TagDatabase::new();
        tag_database.tag_files.push(tag_file);

        let command = FindOtherFileCommand::new();

        assert_eq!("/private/Test.cpp", command.execute("find-other-file Test.h", &mut tag_database));
        assert_eq!("/classes/Test.h", command.execute("find-other-file Test.cpp", &mut tag_database));
    }

    #[test]
    fn should_find_other_file_in_multiple_tags() {
        let mut tag_file = TagFile::new();
        tag_file.tags.push(TagDefinition::from_string(file_tag_declaration("/1/2/Test.h")));
        tag_file.tags.push(TagDefinition::from_string(file_tag_declaration("/a/b/TestA.h")));
        tag_file.tags.push(TagDefinition::from_string(file_tag_declaration("/x/y/Test.cpp")));
        tag_file.tags.push(TagDefinition::from_string(file_tag_declaration("/ma/sogetsu/TestA.cpp")));
        
        let mut tag_database = TagDatabase::new();
        tag_database.tag_files.push(tag_file);
        
        let command = FindOtherFileCommand::new();
        
        assert_eq!("/ma/sogetsu/TestA.cpp", command.execute("find-other-file TestA.h", &mut tag_database));
        assert_eq!("/1/2/Test.h", command.execute("find-other-file /x/y/Test.cpp", &mut tag_database));
    }

    fn file_tag_declaration(file_path: &str) -> String {
        let path = Path::new(file_path);
        format!("{}\t{}\t1;\"\tF\tline:1", path.file_name().unwrap().to_str().unwrap(), file_path)
    }
}
