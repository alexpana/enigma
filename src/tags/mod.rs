pub mod tag_definition;
pub mod tag_file;

// use self::tag_definition::TagDefinition;

#[allow(dead_code)]
pub type TagDefinition = self::tag_definition::TagDefinition;
pub type TagFile = self::tag_file::TagFile;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
#[allow(dead_code)]
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

pub fn tag_kind_from_char(tag_kind: char) -> TagKind {
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

pub struct TagDatabase {
    pub tag_files: Vec<TagFile>
}

impl TagDatabase {
    pub fn new() -> TagDatabase {
        TagDatabase {
            tag_files: Vec::new(),
        }
    }
}
