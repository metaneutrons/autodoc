use crate::config::{DocumentMetadata, MarkdownFile, ProjectConfig};
use crate::discovery::MetadataParser;
use crate::errors::{DocPilotError, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info};

pub struct PdfBuilder {
    config: ProjectConfig,
}

pub struct DocxBuilder {
    config: ProjectConfig,
}

pub struct HtmlBuilder {
    config: ProjectConfig,
}

impl PdfBuilder {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }

    pub async fn build(&self, files: &[MarkdownFile], output_path: &Path) -> Result<()> {
        info!("Building PDF: {}", output_path.display());

        // Merge metadata from all files
        let metadata = MetadataParser::merge_metadata(files);

        // Build pandoc arguments
        let args = self.build_pandoc_args(files, output_path, &metadata)?;

        debug!("Pandoc command: pandoc {}", args.join(" "));

        // Execute pandoc
        let output =
            Command::new("pandoc")
                .args(&args)
                .output()
                .map_err(|e| DocPilotError::Build {
                    message: format!("Failed to execute pandoc: {}", e),
                })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DocPilotError::Build {
                message: format!("Pandoc failed: {}", stderr),
            });
        }

        info!("✅ PDF generated successfully: {}", output_path.display());
        Ok(())
    }

    fn build_pandoc_args(
        &self,
        files: &[MarkdownFile],
        output_path: &Path,
        metadata: &DocumentMetadata,
    ) -> Result<Vec<String>> {
        let mut args = vec![
            "--standalone".to_string(),
            "--listings".to_string(),
            "--pdf-engine".to_string(),
            "xelatex".to_string(),
        ];

        // Template detection
        if let Some(template) = self.find_template()? {
            args.push("--template".to_string());
            args.push(template.to_string_lossy().to_string());
            info!("Using template: {}", template.display());
        }

        // Filters
        args.push("--citeproc".to_string());

        // Document structure
        if let Some(division) = &metadata.top_level_division {
            args.push("--top-level-division".to_string());
            args.push(division.clone());
        } else {
            args.push("--top-level-division".to_string());
            args.push("section".to_string());
        }

        if metadata.numbersections.unwrap_or(true) {
            args.push("--number-sections".to_string());
        }

        // Add metadata arguments
        self.add_metadata_args(&mut args, metadata)?;

        // Input files
        for file in files {
            args.push(file.path.to_string_lossy().to_string());
        }

        // Output
        args.push("-o".to_string());
        args.push(output_path.to_string_lossy().to_string());

        Ok(args)
    }

    fn add_metadata_args(&self, args: &mut Vec<String>, metadata: &DocumentMetadata) -> Result<()> {
        // Essential metadata for Eisvogel template
        if let Some(title) = &metadata.title {
            args.push("--metadata".to_string());
            args.push(format!("title={}", title));
        } else {
            args.push("--metadata".to_string());
            args.push("title=Document".to_string());
        }

        if let Some(author) = &metadata.author {
            let author_str = if author.len() == 1 {
                author[0].clone()
            } else {
                author.join(", ")
            };
            args.push("--metadata".to_string());
            args.push(format!("author={}", author_str));
        }

        if let Some(date) = &metadata.date {
            args.push("--metadata".to_string());
            args.push(format!("date={}", date));
        }

        // Language
        if let Some(lang) = &metadata.lang {
            args.push("--metadata".to_string());
            args.push(format!("lang={}", lang));

            // Auto-detect babel language if not specified
            if metadata.babel_lang.is_none() {
                let babel_lang = self.detect_babel_lang(lang);
                args.push("--metadata".to_string());
                args.push(format!("babel-lang={}", babel_lang));
            }
        }

        if let Some(babel_lang) = &metadata.babel_lang {
            args.push("--metadata".to_string());
            args.push(format!("babel-lang={}", babel_lang));
        }

        // Document class
        if let Some(doc_class) = &metadata.documentclass {
            args.push("--metadata".to_string());
            args.push(format!("documentclass={}", doc_class));
        }

        // Book mode
        if metadata.book.unwrap_or(false) {
            args.push("--metadata".to_string());
            args.push("book=true".to_string());
        }

        Ok(())
    }

    fn detect_babel_lang(&self, lang: &str) -> String {
        match lang.split('-').next().unwrap_or(lang) {
            "de" => "ngerman",
            "fr" => "french",
            "es" => "spanish",
            "it" => "italian",
            "pt" => "portuguese",
            "nl" => "dutch",
            "ru" => "russian",
            _ => "english",
        }
        .to_string()
    }

    fn find_template(&self) -> Result<Option<PathBuf>> {
        let eisvogel_path = self.config.templates_dir.join("eisvogel.latex");

        if eisvogel_path.exists() {
            return Ok(Some(eisvogel_path));
        }

        // Look for any .latex template
        if self.config.templates_dir.exists() {
            for entry in fs::read_dir(&self.config.templates_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().is_some_and(|ext| ext == "latex") {
                    return Ok(Some(path));
                }
            }
        }

        Ok(None)
    }

    pub fn ensure_output_dir(&self) -> Result<()> {
        if !self.config.output_dir.exists() {
            fs::create_dir_all(&self.config.output_dir)?;
            info!(
                "Created output directory: {}",
                self.config.output_dir.display()
            );
        }
        Ok(())
    }
}
impl DocxBuilder {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }

    pub async fn build(&self, files: &[MarkdownFile], output_path: &Path) -> Result<()> {
        info!("Building DOCX: {}", output_path.display());

        let metadata = MetadataParser::merge_metadata(files);
        let args = self.build_pandoc_args(files, output_path, &metadata)?;

        debug!("Pandoc command: pandoc {}", args.join(" "));

        let output =
            Command::new("pandoc")
                .args(&args)
                .output()
                .map_err(|e| DocPilotError::Build {
                    message: format!("Failed to execute pandoc: {}", e),
                })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DocPilotError::Build {
                message: format!("Pandoc failed: {}", stderr),
            });
        }

        info!("✅ DOCX generated successfully: {}", output_path.display());
        Ok(())
    }

    fn build_pandoc_args(
        &self,
        files: &[MarkdownFile],
        output_path: &Path,
        metadata: &DocumentMetadata,
    ) -> Result<Vec<String>> {
        let mut args = Vec::new();

        args.push("--standalone".to_string());
        args.push("--to".to_string());
        args.push("docx".to_string());

        if let Some(template) = self.find_docx_template()? {
            args.push("--reference-doc".to_string());
            args.push(template.to_string_lossy().to_string());
        }

        args.push("--citeproc".to_string());

        if metadata.numbersections.unwrap_or(true) {
            args.push("--number-sections".to_string());
        }

        for file in files {
            args.push(file.path.to_string_lossy().to_string());
        }

        args.push("-o".to_string());
        args.push(output_path.to_string_lossy().to_string());

        Ok(args)
    }

    fn find_docx_template(&self) -> Result<Option<PathBuf>> {
        if self.config.templates_dir.exists() {
            for entry in fs::read_dir(&self.config.templates_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().is_some_and(|ext| ext == "docx") {
                    return Ok(Some(path));
                }
            }
        }
        Ok(None)
    }

    pub fn ensure_output_dir(&self) -> Result<()> {
        if !self.config.output_dir.exists() {
            fs::create_dir_all(&self.config.output_dir)?;
        }
        Ok(())
    }
}

impl HtmlBuilder {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }

    pub async fn build(&self, files: &[MarkdownFile], output_path: &Path) -> Result<()> {
        info!("Building HTML: {}", output_path.display());

        let metadata = MetadataParser::merge_metadata(files);
        let args = self.build_pandoc_args(files, output_path, &metadata)?;

        debug!("Pandoc command: pandoc {}", args.join(" "));

        let output =
            Command::new("pandoc")
                .args(&args)
                .output()
                .map_err(|e| DocPilotError::Build {
                    message: format!("Failed to execute pandoc: {}", e),
                })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DocPilotError::Build {
                message: format!("Pandoc failed: {}", stderr),
            });
        }

        info!("✅ HTML generated successfully: {}", output_path.display());
        Ok(())
    }

    fn build_pandoc_args(
        &self,
        files: &[MarkdownFile],
        output_path: &Path,
        metadata: &DocumentMetadata,
    ) -> Result<Vec<String>> {
        let mut args = vec![
            "--standalone".to_string(),
            "--to".to_string(),
            "html5".to_string(),
            "--self-contained".to_string(),
            "--citeproc".to_string(),
        ];

        if metadata.numbersections.unwrap_or(true) {
            args.push("--number-sections".to_string());
        }

        if let Some(template) = self.find_html_template()? {
            args.push("--template".to_string());
            args.push(template.to_string_lossy().to_string());
        }

        for file in files {
            args.push(file.path.to_string_lossy().to_string());
        }

        args.push("-o".to_string());
        args.push(output_path.to_string_lossy().to_string());

        Ok(args)
    }

    fn find_html_template(&self) -> Result<Option<PathBuf>> {
        if self.config.templates_dir.exists() {
            for entry in fs::read_dir(&self.config.templates_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().is_some_and(|ext| ext == "html") {
                    return Ok(Some(path));
                }
            }
        }
        Ok(None)
    }

    pub fn ensure_output_dir(&self) -> Result<()> {
        if !self.config.output_dir.exists() {
            fs::create_dir_all(&self.config.output_dir)?;
        }
        Ok(())
    }
}
