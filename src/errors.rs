use thiserror::Error;

#[derive(Debug, Error)]
pub enum AutoDocError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("Directory traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Build error: {message}")]
    Build { message: String },
    
    #[error("Dependency missing: {tool} - {hint}")]
    Dependency { tool: String, hint: String },
    
    #[error("File not found: {path}")]
    FileNotFound { path: String },
}

pub type Result<T> = std::result::Result<T, AutoDocError>;
