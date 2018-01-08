use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

pub struct TagFile {
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
            lines
        }
    }
}

pub struct TagDatabase<'a> {
    tags: Vec<TagDefinition<'a>>,
}

impl<'a> TagDatabase<'a> {
    pub fn new() -> TagDatabase<'a> {
        TagDatabase {
            tags: Vec::new(),
        }
    }

    pub fn parse_file<'b:'a> (self: &mut TagDatabase<'a>, tag_file: &'b TagFile) {
        for line in &tag_file.lines {
            if !line.starts_with("!_") {
                let tag_definition = parse_tag_definition(line);
                self.tags.push(tag_definition);
            }
        }
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
