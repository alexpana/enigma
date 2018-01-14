mod tags;
// mod server;
// mod commands;
// mod search;

// use tags::TagDatabase;
// use tags::TagFile;
use tags::tag_definition::TagDefinition;
// use server::Server;
use std::{thread, time};

fn main() {
    // let tag_file = TagFile::from_file("test/tags");
    
    // let mut tags = TagDatabase::new();
    // tags.tag_files.push(tag_file);

    // let mut server = Server::new(&mut tags);

    // server.add_command(Box::new(commands::echo::EchoCommand {}));
    // server.add_command(Box::new(commands::describe_tag::DescribeTag {}));
    // server.add_command(Box::new(commands::find_other_file::FindOtherFileCommand::new()));
    // server.bind("127.0.0.1", 9092);
    let tag_file = tags::tag_file::TagFile::from_file("D:/Unreal/UE_4.17/Engine/Source/Runtime/tags");
    thread::sleep(time::Duration::from_millis(100000));
    assert_eq!(true, false);

}
