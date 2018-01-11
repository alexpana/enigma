use tags::TagDatabase;
use server::ServerCommand;

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