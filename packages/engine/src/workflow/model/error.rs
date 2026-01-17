#[derive(Debug)]
pub enum WorkflowError {
    TomlParseError(String),
    MissingField(&'static str),
    UnknownField(String),
    InvalidType { field: String, expected: String },
    InvalidValue { field: String, reason: String },
    ValidationError(String),
}
