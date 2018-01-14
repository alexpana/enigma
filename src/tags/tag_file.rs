use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

use tags::tag_definition::TagDefinition;

#[allow(dead_code)]
pub struct TagFile {
    file_path: String,
    pub tags: Vec<TagDefinition>
}

impl TagFile {
    #[allow(dead_code)]
    pub fn new() -> TagFile {
        TagFile {
            file_path: "".to_string(),
            tags: Vec::new(),
        }
    }

    pub fn from_file(input_file_path: &str) -> TagFile {
        let mut result = TagFile {
            file_path: String::from(input_file_path),
            tags: Vec::new(),
        };

        let f = File::open(input_file_path).unwrap();
        let reader = BufReader::new(&f);
        let now = Instant::now();
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
        println!("Finished parsing {} tags file in {:.3}s", result.tags.len(), elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9_f64);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    
    #[test]
    fn should_parse_simple_file() {
    }
}
