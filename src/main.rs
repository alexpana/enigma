use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello World!");
    
    let mut input_file = File::open("tags").expect("file not found");
    
    let mut contents = String::new();
    
    input_file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    for line in contents.lines() {

        // Skip header lines
        if line.starts_with("!_") {
            continue;
        }

        let tagaddress_start = line.find("/^").expect("Could not parse tag file");
        let tagaddress_end = line.find("$/;\"").expect("Could not parse tag file") + 4;

        println!("tagaddress: {}", &line[tagaddress_start..tagaddress_end]);
        
        let tag_definition: Vec<&str> = line.split("\t").collect();

        println!("{}", tag_definition[0]);
        
        for tag_info in tag_definition {
            println!("  {}", tag_info);
        }
    }
}
