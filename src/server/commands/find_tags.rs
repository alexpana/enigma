use getopts::Options;

use tags::TagDatabase;
use tags::TagDefinition;
use tags::tag_kind_from_str;

use search::match_abbrev;

use server::ServerCommand;
use server::commands::args::split_args;

pub struct FindTagsCommand {
    options: Options
}

impl FindTagsCommand {
    pub fn new() -> FindTagsCommand {
        let mut options = Options::new();
        
        FindTagsCommand {
            options,
        }
    }

    fn do_execute<'a>(&self,
                  tag_name: &str,
                  all_tags: Box<Iterator<Item=&TagDefinition> + 'a>) -> String {
        for tag in all_tags {
            if (tag.name().starts_with(tag_name)) {
                return tag.to_elisp()
            }
        }
        
        "nil".to_string()
    }
}

impl ServerCommand for FindTagsCommand {
    fn can_execute(&self, command: &str) -> bool {
        command.starts_with("find ")
    }

    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();

        let args = split_args(command);
        let tag_name = args.last().unwrap();

        return self.do_execute(tag_name,
                               tag_database.all_tags());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tags::TagDefinition;
    
    #[test]
    fn finds_one_tag() {
        let command = FindTagsCommand::new();

        let tag_str = "Test	test/Test.h	/^class Test {$/;\"	c\tline:10".to_string();
        
        let tags = vec!(
            TagDefinition::from_string("DoTest\ttest/Test.h\t/^	int DoTest() const {}$/;\"\tf\tclass:Test	typeref:typename:int".to_string()),
            TagDefinition::from_string("Test\ttest/Test.h\t/^class Test {$/;\"\tc\tline:10".to_string())
        );
        assert_eq!(tags.first().unwrap().to_elisp(), command.do_execute("DoTest", Box::new(tags.iter())));
    }
}
