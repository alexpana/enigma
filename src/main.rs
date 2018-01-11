mod tags;
mod server;
mod commands;
mod search;

use tags::TagDatabase;
use tags::TagFile;
use server::Server;

use std::time::Instant;


fn main() {
    let now = Instant::now();
    let tag_file = TagFile::from_file("test/tags");

    let mut tags = TagDatabase::new();
    tags.parse_file(&tag_file);

    let elapsed = now.elapsed();
    println!("# Finished parsing {} tags file in {:.3}s", tags.len(), elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9_f64);

    let mut server = Server::new();

    server.add_command(Box::new(commands::echo::EchoCommand {}));
    server.add_command(Box::new(commands::describe_tag::DescribeTag { tag_database: &tags }));
    server.add_command(Box::new(commands::find_other_file::FindOtherFileCommand::new(&tags)));

    server.bind("127.0.0.1", 9092);
}
