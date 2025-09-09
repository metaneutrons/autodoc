use crate::config::ProjectConfig;
use crate::errors::{AutoDocError, Result};
use reqwest;
use serde_json;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use tracing::{debug, info};
use zip::ZipArchive;

pub struct ProjectInitializer {
    project_name: String,
}

impl ProjectInitializer {
    pub fn new(project_name: String) -> Self {
        Self { project_name }
    }

    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing AutoDoc project: {}", self.project_name);

        // Create directory structure
        self.create_directory_structure().await?;

        // Try to download Eisvogel template (optional)
        if let Err(e) = self.download_eisvogel_template().await {
            tracing::warn!("Failed to download Eisvogel template: {}", e);
            info!("Continuing without template - you can download it later with 'autodoc templates download-eisvogel'");
        }

        // Create configuration file
        self.create_setup_file().await?;

        // Create sample content
        self.create_sample_content().await?;

        info!("✅ Project initialized successfully!");
        println!("🚀 Project '{}' initialized!", self.project_name);
        println!();
        println!("Next steps:");
        println!("  1. Edit 00-setup.md to configure your document");
        println!("  2. Add your content in numbered markdown files");
        println!("  3. Run 'autodoc build pdf' to generate your document");

        Ok(())
    }

    async fn create_directory_structure(&self) -> Result<()> {
        let dirs = ["output", "templates", "images"];

        for dir in &dirs {
            fs::create_dir_all(dir)?;
            debug!("Created directory: {}", dir);
        }

        info!("Created project directory structure");
        Ok(())
    }

    async fn download_eisvogel_template(&self) -> Result<()> {
        info!("Downloading Eisvogel template...");

        let url = "https://github.com/Wandmalfarbe/pandoc-latex-template/releases/latest/download/Eisvogel.zip";
        let response = reqwest::get(url).await.map_err(|e| AutoDocError::Build {
            message: format!("HTTP request failed: {}", e),
        })?;

        if !response.status().is_success() {
            return Err(AutoDocError::Build {
                message: format!(
                    "Failed to download Eisvogel template: HTTP {}",
                    response.status()
                ),
            });
        }

        let bytes = response.bytes().await.map_err(|e| AutoDocError::Build {
            message: format!("Failed to read response: {}", e),
        })?;

        // Create temporary directory for extraction
        let temp_dir = TempDir::new()?;
        let zip_path = temp_dir.path().join("eisvogel.zip");
        fs::write(&zip_path, &bytes)?;

        // Extract the template
        let file = fs::File::open(&zip_path)?;
        let mut archive = ZipArchive::new(file).map_err(|e| AutoDocError::Build {
            message: format!("Failed to open Eisvogel archive: {}", e),
        })?;

        // Find and extract the .latex file
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| AutoDocError::Build {
                message: format!("Failed to read archive entry: {}", e),
            })?;
            if file.name().ends_with(".latex") {
                let mut contents = Vec::new();
                std::io::copy(&mut file, &mut contents)?;

                let template_path = Path::new("templates").join("eisvogel.latex");
                fs::write(&template_path, contents)?;

                info!(
                    "✅ Downloaded Eisvogel template to: {}",
                    template_path.display()
                );
                return Ok(());
            }
        }

        Err(AutoDocError::Build {
            message: "No .latex file found in Eisvogel archive".to_string(),
        })
    }

    async fn create_setup_file(&self) -> Result<()> {
        let current_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

        let setup_content = format!(
            r#"---
# Document Metadata
title: "{}"
author: ["Your Name"]
date: "{}"
# subtitle: "Document Subtitle"

# Language and Localization
lang: "en"
# babel-lang: "ngerman"

# Document Structure
top-level-division: "section"
numbersections: true
# secnumdepth: 3
# toc: true
# toc-depth: 3
# lof: true
# lot: true

# Document Class and Layout
# documentclass: "article"
# classoption: ["11pt", "a4paper"]
# geometry: ["margin=2.5cm"]
# fontsize: "11pt"
# linestretch: 1.2

# Fonts (requires XeLaTeX)
# mainfont: "Times New Roman"
# sansfont: "Arial"
# monofont: "Courier New"
# mathfont: "Latin Modern Math"

# Headers and Footers
# header-left: "Document Title"
# header-center: ""
# header-right: "\\today"
# footer-left: "Author Name"
# footer-center: ""
# footer-right: "\\thepage"

# Bibliography and Citations
# bibliography: "references.bib"
# csl: "ieee.csl"
# link-citations: true
# reference-section-title: "References"

# Code Highlighting
# highlight-style: "github"
# listings: true

# Links and Cross-references
# linkcolor: "blue"
# urlcolor: "blue"
# citecolor: "blue"

# PDF-specific Options
# colorlinks: true
# bookmarks: true
# bookmarksnumbered: true
# pdfcreator: "AutoDoc"
# pdfproducer: "Pandoc with XeLaTeX"

# Eisvogel Template Options
# titlepage: true
# titlepage-color: "FFFFFF"
# titlepage-text-color: "000000"
# titlepage-rule-color: "000000"
# titlepage-rule-height: 2
# logo: "images/logo.png"
# logo-width: "100"
# disable-header-and-footer: false

# Table Options
# table-use-row-colors: true

# Figure Options
# fig-caption-location: "bottom"
# tbl-caption-location: "top"

# Custom Variables
# company: "Your Company"
# department: "Your Department"
# version: "1.0"
# status: "Draft"
---

# Project Setup

This document serves as the main configuration file for your AutoDoc project.
All document settings are defined in the YAML frontmatter above.

## Configuration Guide

### Essential Settings
- **title**: Document title (appears on title page)
- **author**: List of authors (use array format)
- **date**: Document date (use YYYY-MM-DD format)
- **lang**: Document language (en, de, fr, es, etc.)

### Layout Options
- **geometry**: Page margins and layout
- **fontsize**: Base font size (10pt, 11pt, 12pt)
- **numbersections**: Enable section numbering
- **toc**: Enable table of contents

### Advanced Features
- **bibliography**: Reference file for citations
- **highlight-style**: Code syntax highlighting theme
- **titlepage**: Enable custom title page (Eisvogel)

## Next Steps

1. Customize the frontmatter above for your document
2. Add content in numbered markdown files
3. Place images in the images/ directory
4. Run autodoc build pdf to generate your document

For more information, visit: https://github.com/metaneutrons/autodoc
"#,
            self.project_name, current_date
        );

        fs::write("00-setup.md", setup_content)?;
        info!("Created comprehensive 00-setup.md configuration file");

        Ok(())
    }

    async fn create_sample_content(&self) -> Result<()> {
        if !Path::new("01-introduction.md").exists() {
            let sample_content = format!(
                r#"# Introduction

Welcome to your new AutoDoc project: **{}**!

This is a sample introduction file. You can edit this content or create additional numbered markdown files to build your document.

## Getting Started

1. Edit the metadata in 00-setup.md
2. Add your content in numbered markdown files
3. Run autodoc build pdf to generate your document

## Features

AutoDoc provides:
- Professional PDF generation with LaTeX
- Multi-format output (PDF, DOCX, HTML)
- Template management
- Dependency validation
- Multi-language support

Happy writing! 📝
"#,
                self.project_name
            );

            fs::write("01-introduction.md", sample_content)?;
            info!("Created sample content file: 01-introduction.md");
        }

        Ok(())
    }
}

pub fn initialize_project(name: &str) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let initializer = ProjectInitializer::new(name.to_string());
    rt.block_on(initializer.initialize())
}
