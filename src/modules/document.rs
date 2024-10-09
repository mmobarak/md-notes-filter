use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FrontmatterValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Enum(String), // Represented as a string for simplicity
}

#[derive(Debug, Clone)]
pub struct MarkdownDocument {
    pub frontmatter: Option<HashMap<String, FrontmatterValue>>,
    pub content: String,
}

impl MarkdownDocument {
    pub fn new(frontmatter: Option<HashMap<String, FrontmatterValue>>, content: String) -> Self {
        MarkdownDocument {
            frontmatter,
            content,
        }
    }
}
