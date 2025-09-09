use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    // Standard Pandoc metadata
    pub title: Option<String>,
    pub author: Option<Vec<String>>,
    pub date: Option<String>,
    pub subtitle: Option<String>,

    // Language and localization
    pub lang: Option<String>,
    pub babel_lang: Option<String>,

    // Document structure
    pub top_level_division: Option<String>,
    pub numbersections: Option<bool>,
    pub secnumdepth: Option<u8>,
    pub toc: Option<bool>,
    pub toc_depth: Option<u8>,
    pub lof: Option<bool>,
    pub lot: Option<bool>,

    // Document class and layout
    pub documentclass: Option<String>,
    pub classoption: Option<Vec<String>>,
    pub geometry: Option<Vec<String>>,
    pub fontsize: Option<String>,
    pub mainfont: Option<String>,
    pub sansfont: Option<String>,
    pub monofont: Option<String>,

    // Bibliography
    pub bibliography: Option<Vec<String>>,
    pub csl: Option<String>,
    pub link_citations: Option<bool>,

    // PDF-specific
    pub colorlinks: Option<bool>,
    pub linkcolor: Option<String>,
    pub urlcolor: Option<String>,
    pub citecolor: Option<String>,
    pub book: Option<bool>,

    // Custom metadata (extensible)
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub output_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub images_dir: PathBuf,
    pub exclude_files: Vec<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "document".to_string(),
            output_dir: PathBuf::from("output"),
            templates_dir: PathBuf::from("templates"),
            images_dir: PathBuf::from("images"),
            exclude_files: vec!["README.md".to_string()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownFile {
    pub path: PathBuf,
    pub metadata: DocumentMetadata,
    pub content: String,
    pub has_inline_mermaid: bool,
    pub dependencies: Vec<PathBuf>,
    pub last_modified: SystemTime,
}

#[derive(Debug, Clone)]
pub struct DiscoveredFiles {
    pub markdown_files: Vec<MarkdownFile>,
    pub mermaid_files: Vec<PathBuf>,
    pub image_files: Vec<PathBuf>,
    pub template_files: Vec<PathBuf>,
    pub bibliography_files: Vec<PathBuf>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_project_config_default() {
        let config = ProjectConfig::default();
        assert_eq!(config.name, "document");
        assert_eq!(config.output_dir, PathBuf::from("output"));
        assert_eq!(config.templates_dir, PathBuf::from("templates"));
    }

    #[test]
    fn test_document_metadata_default() {
        let metadata = DocumentMetadata::default();
        assert!(metadata.title.is_none());
        assert!(metadata.author.is_none());
        assert!(metadata.date.is_none());
        assert_eq!(metadata.numbersections, Some(true));
    }

    #[test]
    fn test_markdown_file_creation() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let markdown_file = MarkdownFile {
            path: file_path.clone(),
            content: "# Test".to_string(),
            has_inline_mermaid: false,
            dependencies: vec![],
            last_modified: std::time::SystemTime::now(),
            metadata: DocumentMetadata::default(),
        };

        assert_eq!(markdown_file.path, file_path);
        assert_eq!(markdown_file.content, "# Test");
        assert!(!markdown_file.has_inline_mermaid);
    }

    #[test]
    fn test_discovered_files_empty() {
        let files = DiscoveredFiles {
            markdown_files: vec![],
            mermaid_files: vec![],
            image_files: vec![],
            template_files: vec![],
            bibliography_files: vec![],
        };

        assert!(files.markdown_files.is_empty());
        assert!(files.mermaid_files.is_empty());
        assert!(files.image_files.is_empty());
    }
}
