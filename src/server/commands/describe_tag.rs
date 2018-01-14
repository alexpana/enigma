use tags::TagDatabase;
use server::ServerCommand;

pub struct DescribeTag {
}

impl ServerCommand for DescribeTag {
    fn command_name(&self) -> String {
        String::from("describe")
    }

    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();

        let tag_name = tokens[1].trim();

        for tag in tag_database.tags.values().flat_map(|v| v) {
            if tag.name.starts_with(tag_name) {
                return format!("{:?}", tag);
            }
        }

        String::from(format!("Tag {} not found.", tag_name))
    }
}
