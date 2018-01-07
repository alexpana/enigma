extern crate scaproust;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;
use std::time::Duration;
use std::thread;
use scaproust::*;

#[derive(Debug, PartialOrd, PartialEq)]
enum TagKind {
    MacroDefinitions,
    EnumValue,
    FunctionDefinition,
    Enum,
    HeaderInclude,
    LocalVariable,
    ClassMember,
    FunctionPrototype,
    Struct,
    Typedef,
    Union,
    Variable,
    ForwardDeclaration,
    FunctionParameter,
    GotoLabel,
    Class,
    Namespace,
    NamespaceAlias,
    NamespaceUsingStatement,
    File,
    Unknown,
}

fn tag_kind_from_char(tag_kind: char) -> TagKind {
    match tag_kind {
        'd' => TagKind::MacroDefinitions,
        'e' => TagKind::EnumValue,
        'f' => TagKind::FunctionDefinition,
        'g' => TagKind::Enum,
        'h' => TagKind::HeaderInclude,
        'l' => TagKind::LocalVariable,
        'm' => TagKind::ClassMember,
        'p' => TagKind::FunctionPrototype,
        's' => TagKind::Struct,
        't' => TagKind::Typedef,
        'u' => TagKind::Union,
        'v' => TagKind::Variable,
        'x' => TagKind::ForwardDeclaration,
        'z' => TagKind::FunctionParameter,
        'L' => TagKind::GotoLabel,
        'c' => TagKind::Class,
        'n' => TagKind::Namespace,
        'A' => TagKind::NamespaceAlias,
        'N' => TagKind::NamespaceUsingStatement,
        'U' => TagKind::NamespaceUsingStatement,
        'F' => TagKind::File,
        _ => TagKind::Unknown
    }
}

#[derive(Debug)]
struct TagLocation<'a> {
    file_path: &'a str,
    line: usize,
}

#[derive(Debug)]
struct TagDefinition<'a> {
    name: &'a str,
    declaration: &'a str,
    location: TagLocation<'a>,
    kind: TagKind,
    fields: Vec<&'a str>,
}

fn parse_declaration(line: &str) -> (&str, usize) {
    let declaration_start_find = line.find("/^");
    match declaration_start_find {
        None => {
            ("", line.find(";\"").expect("Could not parse tag file") + 3)
        }
        Some(v) => {
            let declaration_end = line.find(";\"").expect("Could not parse tag file") + 4;
            (&line[v + 2..declaration_end - 4], declaration_end + 1)
        }
    }
}

fn parse_tag_definition(line: &String) -> TagDefinition {
    let expect_msg = "Could not parse tag file";

    let name = line.split("\t").nth(0).expect(expect_msg);

    let location_file_path = line.split("\t").nth(1).expect(expect_msg);

    let (declaration, cursor) = parse_declaration(&line[..]);

    let tag_kind_char = line.chars().nth(cursor).unwrap();

    let fields: Vec<&str> = line[cursor + 1..].split("\t").filter(|x| x.len() > 0).collect();

    let location_line = {
        let line_field = fields.iter().find(|x| x.starts_with("line:"));
        match line_field {
            None => 0 as usize,
            Some(v) => v[v.find(":").unwrap() + 1..].parse::<usize>().unwrap()
        }
    };

    TagDefinition {
        name,
        declaration,
        location: TagLocation {
            file_path: location_file_path,
            line: location_line,
        },
        kind: tag_kind_from_char(tag_kind_char),
        fields,
    }
}

fn main() {
    let now = Instant::now();

    let mut lines: Vec<String> = Vec::new();
    let mut tags: Vec<TagDefinition> = Vec::new();

    {
        let input_file_path = "D:/Unreal/UE_4.17/Engine/Source/Runtime/tags";
        let f = File::open(input_file_path).unwrap();
        let reader = BufReader::new(&f);

        for (num, line) in reader.lines().enumerate() {
            match line {
                Err(e) => {
                    println!("Error reading line {}: {}", num, e);
                }
                Ok(v) => {
                    lines.push(v);
                }
            }
        }
    }

    for line in &lines {
        if !line.starts_with("!_") {
            let tag_definition = parse_tag_definition(line);
            tags.push(tag_definition);
        }
    }

    let elapsed = now.elapsed();

    println!("Finished parsing {} tags file in {:.3}s", tags.len(), elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9_f64);
}
