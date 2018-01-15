use getopts::Options;

use tags::TagDatabase;
use tags::tag_kind_from_str;
use server::ServerCommand;
use server::commands::args::split_args;

pub struct DescribeTagCommand {
    options: Options
}

impl DescribeTagCommand {
    pub fn new() -> DescribeTagCommand {
        let mut options = Options::new();
        options.optopt("k", "kind", "tag kind", "NAME");
        
        DescribeTagCommand {
            options,
        }
    }

    fn do_execute(&self, tag_name: &str, tag_kind: &str, tag_database: &TagDatabase) -> String {
        let kind = tag_kind_from_str(tag_kind);
        
        for tag in tag_database.all_tags() {
            if tag.kind() == kind && tag.name() == tag_name {
                return tag.to_elisp()
            }
        }
        
        "".to_string()
    }

    fn usage_help(&self) -> String {
        "describe -k <tag_kind> <tag_name>".to_string()
    }
}

impl ServerCommand for DescribeTagCommand {
    fn can_execute(&self, command: &str) -> bool {
        command.starts_with("describe ")
    }

    fn execute(&self, command: &str, tag_database: &mut TagDatabase) -> String {
        let tokens: Vec<&str> = command.lines().nth(0).unwrap().split(" ").collect();

        let args = split_args(command);
        let tag_name = args.last().unwrap().clone();
        let matches = match self.options.parse(args) {
            Ok(m) => m,
            Err(f) => return f.to_string()
        };

        if matches.opt_present("kind") {
            return self.do_execute(tag_name, &matches.opt_str("kind").unwrap(), tag_database);
        }

        return self.usage_help()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn parse_args() {
        // let describe_tag = DescribeTagCommand::new();
        // let args = vec!("-t", "file", "\"/help me/test.h\"");
        // let matches = match describe_tag.options.parse(args) {
        //     Ok(m) => m,
        //     Err(f) => panic!(f.to_string())
        // };

        // if matches.opt_present("type") {
        //     println!("Only Type {:?}", matches.opt_str("type"));
        // }
    }
}
