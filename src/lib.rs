pub mod models;
pub mod parser;
pub mod formatter;
pub mod ui;

pub use models::*;
pub use parser::*;
pub use formatter::*;

pub type Result<T> = std::result::Result<T, TfdiffError>;

#[derive(Debug, thiserror::Error)]
pub enum TfdiffError {
    #[error("Failed to parse Terraform output: {0}")]
    ParseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    
    #[error("Web server error: {0}")]
    WebError(String),
}