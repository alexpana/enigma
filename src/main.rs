mod tags;
mod server;

use tags::TagDatabase;
use tags::TagFile;
use server::Server;

use std::time::Instant;


fn main() {
    let now = Instant::now();
    let tag_file = TagFile::from_file("D:/Unreal/UE_4.17/Engine/Source/Runtime/tags");

    let mut tags = TagDatabase::new();
    tags.parse_file(&tag_file);

    let elapsed = now.elapsed();
    println!("Finished parsing {} tags file in {:.3}s", tags.len(), elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9_f64);

    let mut server = Server {};

    server.bind("localhost", 9092);
}
