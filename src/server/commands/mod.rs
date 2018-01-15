mod echo;
mod find_other_file;
mod load_tags_file;
mod describe_tag;
mod args;

pub type Echo = echo::EchoCommand;
pub type FindOtherFile = find_other_file::FindOtherFileCommand;
pub type LoadTagsFile = load_tags_file::LoadTagsFileCommand;
pub type DescribeTagCommand = describe_tag::DescribeTagCommand;
