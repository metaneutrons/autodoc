use crate::config::{DiscoveredFiles, DocumentMetadata, MarkdownFile, ProjectConfig};
use crate::errors::{AutoDocError, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};
use walkdir::WalkDir;

pub struct FileDiscovery {
    project_config: ProjectConfig,
}

impl FileDiscovery {
    pub fn new(config: ProjectConfig) -> Self {
        Self {
            project_config: config,
        }
    }

    pub fn discover_all(&self) -> Result<DiscoveredFiles> {
        info!("Discovering project files...");

        let markdown_files = self.discover_and_parse_markdown_files()?;
        let mermaid_files = self.discover_mermaid_files()?;
        let image_files = self.discover_image_files()?;
        let template_files = self.discover_template_files()?;
        let bibliography_files = self.discover_bibliography_files()?;

        info!(
            "Found {} markdown files, {} mermaid files, {} images",
            markdown_files.len(),
            mermaid_files.len(),
            image_files.len()
        );

        Ok(DiscoveredFiles {
            markdown_files,
            mermaid_files,
            image_files,
            template_files,
            bibliography_files,
        })
    }

    fn discover_and_parse_markdown_files(&self) -> Result<Vec<MarkdownFile>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(".").max_depth(1) {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "md") && !self.should_exclude(path) {
                debug!("Parsing markdown file: {:?}", path);
                let parsed_file = MetadataParser::parse_file(path)?;
                files.push(parsed_file);
            }
        }

        // Sort files naturally (00-setup.md, 01-intro.md, etc.)
        files.sort_by(|a, b| {
            natord::compare(
                &a.path.file_name().unwrap().to_string_lossy(),
                &b.path.file_name().unwrap().to_string_lossy(),
            )
        });

        Ok(files)
    }

    fn discover_mermaid_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(".").max_depth(2) {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "mmd") {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    fn discover_image_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let image_extensions = ["png", "jpg", "jpeg", "svg", "pdf", "gif", "webp"];

        if self.project_config.images_dir.exists() {
            for entry in WalkDir::new(&self.project_config.images_dir).max_depth(3) {
                let entry = entry?;
                let path = entry.path();

                if let Some(ext) = path.extension() {
                    if image_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(files)
    }

    fn discover_template_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if self.project_config.templates_dir.exists() {
            for entry in WalkDir::new(&self.project_config.templates_dir).max_depth(2) {
                let entry = entry?;
                let path = entry.path();

                if path
                    .extension()
                    .is_some_and(|ext| ext == "latex" || ext == "tex")
                {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }

    fn discover_bibliography_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let bib_extensions = ["bib", "bibtex", "json", "yaml"];

        for entry in WalkDir::new(".").max_depth(2) {
            let entry = entry?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if bib_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }

    fn should_exclude(&self, path: &Path) -> bool {
        let filename = path.file_name().unwrap().to_string_lossy();

        // Check custom exclusions
        self.project_config.exclude_files.iter().any(|pattern| {
            let regex = Regex::new(pattern)
                .unwrap_or_else(|_| Regex::new(&regex::escape(pattern)).unwrap());
            regex.is_match(&filename)
        })
    }
}

pub struct MetadataParser;

impl MetadataParser {
    pub fn parse_file(path: &Path) -> Result<MarkdownFile> {
        let content = fs::read_to_string(path)?;
        let (metadata, content_without_frontmatter) = Self::extract_frontmatter(&content)?;

        let has_inline_mermaid = Self::detect_inline_mermaid(&content_without_frontmatter);
        let dependencies = Self::extract_dependencies(&content_without_frontmatter, path.parent())?;
        let last_modified = fs::metadata(path)?.modified()?;

        Ok(MarkdownFile {
            path: path.to_path_buf(),
            metadata,
            content: content_without_frontmatter,
            has_inline_mermaid,
            dependencies,
            last_modified,
        })
    }

    fn extract_frontmatter(content: &str) -> Result<(DocumentMetadata, String)> {
        if let Some(stripped) = content.strip_prefix("---\n") {
            if let Some(end) = stripped.find("\n---\n") {
                let yaml_content = &stripped[..end];
                let remaining_content = &stripped[end + 5..];

                let metadata: DocumentMetadata =
                    serde_yaml::from_str(yaml_content).map_err(AutoDocError::Yaml)?;

                return Ok((metadata, remaining_content.to_string()));
            }
        }

        Ok((DocumentMetadata::default(), content.to_string()))
    }

    fn detect_inline_mermaid(content: &str) -> bool {
        content.contains("```mermaid")
    }

    fn extract_dependencies(content: &str, base_dir: Option<&Path>) -> Result<Vec<PathBuf>> {
        let mut dependencies = Vec::new();

        // Simple regex-based dependency extraction for now
        let image_regex = Regex::new(r"!\[.*?\]\(([^)]+)\)").unwrap();
        let link_regex = Regex::new(r"\[.*?\]\(([^)]+)\)").unwrap();

        for cap in image_regex.captures_iter(content) {
            if let Some(url) = cap.get(1) {
                let url_str = url.as_str();
                if !url_str.starts_with("http") && !url_str.starts_with("mailto:") {
                    if let Some(base) = base_dir {
                        let path = base.join(url_str);
                        if path.exists() {
                            dependencies.push(path);
                        }
                    }
                }
            }
        }

        for cap in link_regex.captures_iter(content) {
            if let Some(url) = cap.get(1) {
                let url_str = url.as_str();
                if !url_str.starts_with("http") && !url_str.starts_with("mailto:") {
                    if let Some(base) = base_dir {
                        let path = base.join(url_str);
                        if path.exists() && path.extension().is_some_and(|ext| ext == "md") {
                            dependencies.push(path);
                        }
                    }
                }
            }
        }

        Ok(dependencies)
    }

    pub fn merge_metadata(files: &[MarkdownFile]) -> DocumentMetadata {
        let mut merged = DocumentMetadata::default();

        // Priority: 00-setup.md > first file with metadata > defaults
        for file in files {
            if file.path.file_name().unwrap() == "00-setup.md" {
                merged = file.metadata.clone();
                break;
            }
        }

        // Fill in missing values from other files
        for file in files {
            Self::merge_missing(&mut merged, &file.metadata);
        }

        merged
    }

    /// Extract project configuration from Markdown files (replaces autodoc.yml)
    #[allow(dead_code)]
    pub fn extract_project_config(markdown_files: &[MarkdownFile]) -> ProjectConfig {
        let mut config = ProjectConfig::default();

        // Look for configuration in 00-setup.md or first file
        for file in markdown_files {
            if file.path.file_name().and_then(|n| n.to_str()) == Some("00-setup.md")
                || file
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .starts_with("00-")
            {
                // Extract project config from frontmatter
                if let Some(title) = &file.metadata.title {
                    config.name = title.clone();
                }

                // Check for custom output directory in frontmatter or content
                if let Some(output_dir) = file
                    .content
                    .lines()
                    .find(|line| line.starts_with("output_dir:"))
                    .and_then(|line| line.split(':').nth(1))
                    .map(|s| s.trim().trim_matches('"'))
                {
                    config.output_dir = PathBuf::from(output_dir);
                }

                // Check for templates directory
                if let Some(templates_dir) = file
                    .content
                    .lines()
                    .find(|line| line.starts_with("templates_dir:"))
                    .and_then(|line| line.split(':').nth(1))
                    .map(|s| s.trim().trim_matches('"'))
                {
                    config.templates_dir = PathBuf::from(templates_dir);
                }

                break;
            }
        }

        config
    }

    fn merge_missing(target: &mut DocumentMetadata, source: &DocumentMetadata) {
        if target.title.is_none() {
            target.title = source.title.clone();
        }
        if target.author.is_none() {
            target.author = source.author.clone();
        }
        if target.date.is_none() {
            target.date = source.date.clone();
        }
        if target.lang.is_none() {
            target.lang = source.lang.clone();
        }
        if target.babel_lang.is_none() {
            target.babel_lang = source.babel_lang.clone();
        }
        if target.top_level_division.is_none() {
            target.top_level_division = source.top_level_division.clone();
        }
        if target.numbersections.is_none() {
            target.numbersections = source.numbersections;
        }
        if target.documentclass.is_none() {
            target.documentclass = source.documentclass.clone();
        }
        if target.mainfont.is_none() {
            target.mainfont = source.mainfont.clone();
        }
        if target.sansfont.is_none() {
            target.sansfont = source.sansfont.clone();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_discovery_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config = ProjectConfig {
            name: "test".to_string(),
            output_dir: temp_dir.path().join("output"),
            templates_dir: temp_dir.path().join("templates"),
            exclude_files: vec![],
            images_dir: temp_dir.path().join("images"),
        };

        // Change to temp directory for discovery
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let discovery = FileDiscovery::new(config);
        let result = discovery.discover_all();

        // Restore original directory immediately
        std::env::set_current_dir(&original_dir).unwrap();

        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.markdown_files.is_empty());
        assert!(files.mermaid_files.is_empty());
    }

    #[tokio::test]
    async fn test_file_discovery_with_markdown() {
        let temp_dir = TempDir::new().unwrap();
        let md_file = temp_dir.path().join("test.md");
        fs::write(&md_file, "# Test Document").unwrap();

        let config = ProjectConfig {
            name: "test".to_string(),
            output_dir: temp_dir.path().join("output"),
            templates_dir: temp_dir.path().join("templates"),
            exclude_files: vec![],
            images_dir: temp_dir.path().join("images"),
        };

        // Change to temp directory for discovery
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let discovery = FileDiscovery::new(config);
        let result = discovery.discover_all();

        // Restore original directory immediately
        std::env::set_current_dir(&original_dir).unwrap();

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.markdown_files.len(), 1);
        assert!(files.markdown_files[0].path.ends_with("test.md"));
    }

    #[tokio::test]
    async fn test_file_discovery_with_mermaid() {
        let temp_dir = TempDir::new().unwrap();
        let mmd_file = temp_dir.path().join("diagram.mmd");
        fs::write(&mmd_file, "graph TD\n    A --> B").unwrap();

        let config = ProjectConfig {
            name: "test".to_string(),
            output_dir: temp_dir.path().join("output"),
            templates_dir: temp_dir.path().join("templates"),
            exclude_files: vec![],
            images_dir: temp_dir.path().join("images"),
        };

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let discovery = FileDiscovery::new(config);
        let result = discovery.discover_all();

        std::env::set_current_dir(&original_dir).unwrap();

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.mermaid_files.len(), 1);
        assert!(files.mermaid_files[0].ends_with("diagram.mmd"));
    }

    #[test]
    fn test_metadata_parser_empty() {
        let files = vec![];
        let metadata = MetadataParser::merge_metadata(&files);

        assert!(metadata.title.is_none());
        assert!(metadata.author.is_none());
        assert_eq!(metadata.numbersections, Some(true));
    }

    #[test]
    fn test_metadata_parser_with_frontmatter() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");

        let content = r#"---
title: "Test Document"
author: ["Test Author"]
date: "2024-01-01"
---

# Content"#;

        fs::write(&file_path, content).unwrap();

        // Use the proper parsing method
        let markdown_file = MetadataParser::parse_file(&file_path).unwrap();
        let files = vec![markdown_file];
        let metadata = MetadataParser::merge_metadata(&files);

        assert_eq!(metadata.title, Some("Test Document".to_string()));
        assert_eq!(metadata.author, Some(vec!["Test Author".to_string()]));
        assert_eq!(metadata.date, Some("2024-01-01".to_string()));
    }
}
