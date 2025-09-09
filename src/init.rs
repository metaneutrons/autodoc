use crate::config::ProjectConfig;
use crate::errors::{AutoDocError, Result};
use reqwest;
use serde_json;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use tracing::{info, debug};
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
        
        let client = reqwest::Client::new();
        
        // Get latest release info from GitHub API
        let response = client
            .get("https://api.github.com/repos/Wandmalfarbe/pandoc-latex-template/releases/latest")
            .header("User-Agent", "AutoDoc")
            .send()
            .await
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to fetch Eisvogel release info: {}", e) 
            })?;
        
        let release: serde_json::Value = response.json().await
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to parse GitHub API response: {}", e) 
            })?;
        
        let download_url = release["assets"][0]["browser_download_url"]
            .as_str()
            .ok_or_else(|| AutoDocError::Build { 
                message: "No download URL found in GitHub release".to_string() 
            })?;
        
        debug!("Downloading from: {}", download_url);
        
        // Download and extract
        let zip_response = client.get(download_url).send().await
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to download Eisvogel template: {}", e) 
            })?;
        
        let zip_bytes = zip_response.bytes().await
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to read Eisvogel template: {}", e) 
            })?;
        
        let temp_dir = TempDir::new()?;
        let zip_path = temp_dir.path().join("eisvogel.zip");
        fs::write(&zip_path, zip_bytes)?;
        
        // Extract template
        let file = fs::File::open(&zip_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to open Eisvogel archive: {}", e) 
            })?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| AutoDocError::Build { 
                    message: format!("Failed to read archive entry: {}", e) 
                })?;
            
            if file.name().ends_with("eisvogel.latex") && !file.name().contains("multi-file") {
                let mut contents = Vec::new();
                std::io::copy(&mut file, &mut contents)
                    .map_err(|e| AutoDocError::Build { 
                        message: format!("Failed to extract template: {}", e) 
                    })?;
                
                let output_path = Path::new("templates").join("eisvogel.latex");
                fs::write(output_path, contents)?;
                
                info!("✅ Eisvogel template downloaded successfully");
                return Ok(());
            }
        }
        
        Err(AutoDocError::Build { 
            message: "Eisvogel template not found in archive".to_string() 
        })
    }
    
    async fn create_setup_file(&self) -> Result<()> {
        let current_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        let setup_content = format!(r#"---
# Document Metadata
title: "{}"
author: ["Your Name"]
date: "{}"
# Document subtitle
# subtitle: "Document Subtitle"

# Language and Localization
lang: "en"
# For specific babel language
# babel-lang: "ngerman"

# Document Structure
# Use 'section' for articles, 'chapter' for books/reports
top-level-division: "section"
numbersections: true
# Numbering depth (1-6)
# secnumdepth: 3
# Table of contents
# toc: true
# TOC depth
# toc-depth: 3
# List of figures
# lof: true
# List of tables
# lot: true

# Document Class and Layout
# Options: article, book, report, scrartcl, scrbook, scrreprt
# documentclass: "article"
# classoption: ["11pt", "a4paper"]
# geometry: ["margin=2.5cm"]
# fontsize: "11pt"
# mainfont: "Times New Roman"
# Used for headings in Eisvogel template
# sansfont: "Arial"
# monofont: "Courier New"

# Bibliography and Citations
# bibliography: "references.bib"
# Citation style
# csl: "ieee.csl"
# link-citations: true

# PDF-specific Options
# colorlinks: true
# linkcolor: "blue"
# urlcolor: "blue"
# citecolor: "blue"
# Use book class (enables chapters)
# book: true

# Eisvogel Template Options
# titlepage: true
# titlepage-color: "06386e"
# titlepage-text-color: "FFFFFF"
# titlepage-rule-color: "FFFFFF"
# titlepage-rule-height: 1
# titlepage-background: "background.pdf"
# logo: "logo.png"
# logo-width: "100"
# footer-left: "Footer Text"
# header-right: "Header Text"
# disable-header-and-footer: false
# listings-disable-line-numbers: false
# code-block-font-size: "\footnotesize"

# HTML Output Options
# css: "style.css"
# self-contained: true
---
"#, self.project_name, current_date);
        
        fs::write("00-setup.md", setup_content)?;
        info!("Created 00-setup.md configuration file");
        Ok(())
    }
    
    async fn create_sample_content(&self) -> Result<()> {
        if !Path::new("01-introduction.md").exists() {
            let sample_content = format!(r#"# Introduction

Welcome to your new {} document project!

This is a sample introduction section. You can edit this file and add more numbered markdown files to build your document.

## Getting Started

1. Edit the metadata in `00-setup.md`
2. Add your content in numbered markdown files (01-intro.md, 02-chapter.md, etc.)
3. Run `autodoc build pdf` to generate your document

## Features

- **Professional PDF output** with LaTeX typesetting
- **Multiple formats**: PDF, DOCX, HTML (coming soon)
- **Mermaid diagrams** with native Rust rendering (coming soon)
- **Bibliography support** with pandoc-citeproc
- **Template system** with Eisvogel and custom templates

Happy writing!
"#, self.project_name);
            
            fs::write("01-introduction.md", sample_content)?;
            info!("Created sample content file: 01-introduction.md");
        }
        
        Ok(())
    }
}
