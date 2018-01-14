use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

use tags::tag_definition::TagDefinition;

pub struct TagFile {
    file_path: String,
    tags: Vec<TagDefinition>
}

impl TagFile {
    pub fn from_file(input_file_path: &str) -> TagFile {
        let mut result = TagFile {
            file_path: String::from(input_file_path),
            tags: Vec::new(),
        };

        let now = Instant::now();
        let f = File::open(input_file_path).unwrap();
        let reader = BufReader::new(&f);

        for (num, line) in reader.lines().enumerate() {
            match line {
                Err(e) => {
                    println!("Error reading line {}: {}", num, e);
                }
                Ok(v) => {
                    if !v.starts_with("!_") {
                        result.tags.push(TagDefinition::from_string(v));
                    }
                }
            }
        }

        let elapsed = now.elapsed();
        println!("# Finished parsing {} tags file in {:.3}s", result.tags.len(), elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9_f64);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    
    #[test]
    fn should_parse_simple_file() {
        let tag_file = TagFile::from_file("D:/Unreal/UE_4.17/Engine/Source/Runtime/tags");
        thread::sleep(time::Duration::from_millis(100000));
        assert_eq!(true, false);
    }
}
