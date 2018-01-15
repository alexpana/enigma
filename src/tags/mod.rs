pub mod tag_definition;
pub mod tag_file;

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

pub fn tag_kind_from_str(tag_kind: &str) -> TagKind {
    match &tag_kind.to_lowercase()[..] {
        "macro" => TagKind::MacroDefinitions,
        "enum_value" => TagKind::EnumValue,
        "function" => TagKind::FunctionDefinition,
        "enum" => TagKind::Enum,
        "header_include" => TagKind::HeaderInclude,
        "local_variable" => TagKind::LocalVariable,
        "member" => TagKind::ClassMember,
        "function_prototype" => TagKind::FunctionPrototype,
        "struct" => TagKind::Struct,
        "typedef" => TagKind::Typedef,
        "union" => TagKind::Union,
        "variable" => TagKind::Variable,
        "forward_declaration" => TagKind::ForwardDeclaration,
        "function_parameter" => TagKind::FunctionParameter,
        "goto_label" => TagKind::GotoLabel,
        "class" => TagKind::Class,
        "namespace" => TagKind::Namespace,
        "namespace_alias" => TagKind::NamespaceAlias,
        "namespace_using_statement" => TagKind::NamespaceUsingStatement,
        "file" => TagKind::File,
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

    pub fn all_tags<'a>(&'a self) -> Box<Iterator<Item = &'a TagDefinition> + 'a> {
        Box::new(self.tag_files.iter().map(|v| &v.tags).flat_map(|v| v))
    }
}
