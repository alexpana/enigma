use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TagKind {
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
pub struct TagLocation<'a> {
    pub file_path: &'a str,
    pub line: usize,
}

impl<'a> TagLocation<'a> {
    pub fn new(file_path: &'a str, line: usize) -> TagLocation<'a> {
        TagLocation {
            file_path,
            line,
        }
    }
}

#[derive(Debug)]
pub struct TagDefinition<'a> {
    pub name: &'a str,
    pub declaration: &'a str,
    pub location: TagLocation<'a>,
    pub kind: TagKind,
    pub fields: Vec<&'a str>,
}

impl<'a> TagDefinition<'a> {
    pub fn new_file(file_path: &str) -> TagDefinition {
        let path = Path::new(file_path);

        TagDefinition {
            name: path.file_name().unwrap().to_str().unwrap(),
            declaration: "",
            location: TagLocation {
                file_path,
                line: 1,
            },
            kind: TagKind::File,
            fields: Vec::new(),
        }
    }
}

pub struct TagFile {
    file_path: String,
    lines: Vec<String>,
}

impl TagFile {
    pub fn from_file(input_file_path: &str) -> TagFile {
        let mut lines = Vec::new();
        {
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

        TagFile {
            file_path: String::from(input_file_path),
            lines,
        }
    }
}

pub struct TagDatabase<'a> {
    pub tags: HashMap<String, Vec<TagDefinition<'a>>>,
}

impl<'a> TagDatabase<'a> {
    pub fn new() -> TagDatabase<'a> {
        TagDatabase {
            tags: HashMap::new(),
        }
    }

    pub fn parse_file<'b: 'a>(self: &mut TagDatabase<'a>, tag_file: &'b TagFile) {
        let mut tags = Vec::new();

        for line in &tag_file.lines {
            if !line.starts_with("!_") {
                let tag_definition = parse_tag_definition(line);
                tags.push(tag_definition);
            }
        }

        self.tags.insert(String::from(tag_file.file_path.as_str()), tags);
    }

    pub fn len(self: &TagDatabase<'a>) -> usize {
        return self.tags.len();
    }
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

fn parse_tag_definition<'a>(line: &'a String) -> TagDefinition<'a> {
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