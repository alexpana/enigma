use super::TagKind;
use super::tag_kind_from_char;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TagDefinition {
    original_line: String,
    name: (usize, usize),
    declaration: (usize, usize),
    // file_name slice and line index
    location: ((usize, usize), usize),
    kind: TagKind,
    fields: Vec<(usize, usize)>,
}

#[allow(dead_code)]
impl TagDefinition {

    pub fn name(&self) -> &str {
        &self.original_line[self.name.0..self.name.1]
    }

    pub fn source_file(&self) -> &str {
        &self.original_line[(self.location.0).0..(self.location.0).1]
    }

    pub fn source_line(&self) -> usize {
        self.location.1
    }

    pub fn kind(&self) -> TagKind {
        self.kind.clone()
    }

    pub fn declaration(&self) -> &str {
        &self.original_line[self.declaration.0..self.declaration.1]
    }
    
    pub fn new() -> TagDefinition {
        TagDefinition {
            original_line: String::new(),
            name: (0, 0),
            declaration: (0, 0),
            location: ((0, 0), 0),
            kind: TagKind::File,
            fields: Vec::new(),
        }
    }
    
    pub fn new_file(file_path: &str) -> TagDefinition {
        TagDefinition {
            original_line: file_path.to_string(),
            name: (0, file_path.len()),
            declaration: (0, 0),
            location: ((0, 0), 0),
            kind: TagKind::File,
            fields: Vec::new(),
        }
    }

    pub fn from_string(tag_definition: String) -> TagDefinition {
        let mut cursor = 0;
        let mut token_end;

        // name
        token_end = tag_definition.find("\t").unwrap();
        let name = (cursor, token_end);
        cursor = token_end + 1;

        // file
        token_end = tag_definition[cursor..].find("\t").unwrap() + cursor;
        let location_file_path = (cursor, token_end);

        // declaration
        let (declaration, mut cursor) = parse_declaration(&tag_definition[..]);

        // kind
        let tag_kind_char = tag_definition.chars().nth(cursor).unwrap();
        cursor = cursor + 1;

        // fields
        let fields: Vec<(usize, usize)> = tuples_from_split(&tag_definition[cursor + 1..], "\t")
            .iter().map(|f| (cursor + 1 + f.0, cursor + 1 + f.1)).collect();

        // line number (from fields)
        let location_line = {
            let line_field = fields.iter().find(|x| tag_definition[(x.0)..(x.1)].starts_with("line:"));
            match line_field {
                None => 1 as usize,
                Some(v) => {
                    let slice = &tag_definition[(v.0)..(v.1)];
                    slice[slice.find(":").unwrap() + 1..].parse::<usize>().unwrap()
                }
            }
        };

        TagDefinition {
            original_line: tag_definition,
            name,
            declaration,
            location: (location_file_path, location_line),
            kind: tag_kind_from_char(tag_kind_char),
            fields,
        }
    }

    pub fn to_elisp(&self) -> String {
        format!("(tag :name \"{}\" :source \"{}\" line: {} :kind '{:?}))", self.name(),
                self.source_file(), self.source_line(), self.kind())
    }
}

fn parse_declaration(line: &str) -> ((usize, usize), usize) {
    let declaration_start_find = line.find("/^");
    match declaration_start_find {
        None => {
            ((0, 0), line.find(";\"").unwrap() + 3)
        }
        Some(v) => {
            let declaration_end = line.find(";\"").unwrap();
            ((v + 2, declaration_end - 2), declaration_end + 3)
        }
    }
}

fn tuples_from_split(value: &str, separator: &str) -> Vec<(usize, usize)> {
    let separator_indices: Vec<usize> = value.match_indices(separator).map(|v| v.0).collect();
    
    if separator_indices.is_empty() {
        if value.is_empty() {
            return Vec::new();
        }
        return vec!((0, value.len()));
    }
    
    let mut slices: Vec<(usize, usize)> = Vec::new();
    slices.push((0, separator_indices[0]));
    for i in 0..(separator_indices.len()-1) {
        slices.push((separator_indices[i], separator_indices[i+1]))
    }
    slices.push((*separator_indices.last().unwrap(), value.len()));
    slices
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn should_parse_file_declaration() {
        let definition_str = "Test.h\t/d/tmp/Test.h\t1;\"\tF\tline:1".to_string();
        let tag_definition = TagDefinition::from_string(definition_str);
        assert_eq!("Test.h", tag_definition.name());
        assert_eq!("/d/tmp/Test.h", tag_definition.source_file());
        assert_eq!(1, tag_definition.source_line());
        assert_eq!(TagKind::File, tag_definition.kind());
        assert_eq!("", tag_definition.declaration());
    }

    #[test]
    fn should_parse_class_declaration() {
        let definition_str = "Test\tTest.h\t/^class Test {$/;\"\tc\tline:13".to_string();
        let tag_definition = TagDefinition::from_string(definition_str);
        assert_eq!("Test", tag_definition.name());
        assert_eq!("Test.h", tag_definition.source_file());
        assert_eq!(13, tag_definition.source_line());
        assert_eq!(TagKind::Class, tag_definition.kind());
        assert_eq!("class Test {", tag_definition.declaration());
    }

}
