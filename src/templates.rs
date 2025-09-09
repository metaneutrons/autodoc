use crate::errors::{AutoDocError, Result};
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, debug};
use reqwest;

pub struct TemplateManager {
    templates_dir: PathBuf,
}

impl TemplateManager {
    pub fn new(templates_dir: PathBuf) -> Self {
        Self { templates_dir }
    }
    
    pub async fn download_eisvogel(&self) -> Result<()> {
        info!("Downloading Eisvogel template");
        
        self.ensure_templates_dir()?;
        
        let url = "https://raw.githubusercontent.com/Wandmalfarbe/pandoc-latex-template/v2.4.2/eisvogel.latex";
        let response = reqwest::get(url).await
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to download template: {}", e) 
            })?;
        
        let content = response.text().await
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to read template content: {}", e) 
            })?;
        
        let template_path = self.templates_dir.join("eisvogel.latex");
        fs::write(&template_path, content)?;
        
        info!("✅ Eisvogel template downloaded: {}", template_path.display());
        Ok(())
    }
    
    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();
        
        if !self.templates_dir.exists() {
            return Ok(templates);
        }
        
        for entry in fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    templates.push(name_str.to_string());
                }
            }
        }
        
        templates.sort();
        Ok(templates)
    }
    
    pub fn install_template(&self, source_path: &Path) -> Result<()> {
        info!("Installing template from: {}", source_path.display());
        
        if !source_path.exists() {
            return Err(AutoDocError::Build { 
                message: format!("Template file not found: {}", source_path.display()) 
            });
        }
        
        self.ensure_templates_dir()?;
        
        let file_name = source_path.file_name()
            .ok_or_else(|| AutoDocError::Build { 
                message: "Invalid template file path".to_string() 
            })?;
        
        let dest_path = self.templates_dir.join(file_name);
        fs::copy(source_path, &dest_path)?;
        
        info!("✅ Template installed: {}", dest_path.display());
        Ok(())
    }
    
    fn ensure_templates_dir(&self) -> Result<()> {
        if !self.templates_dir.exists() {
            fs::create_dir_all(&self.templates_dir)?;
            debug!("Created templates directory: {}", self.templates_dir.display());
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_template_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        
        assert_eq!(manager.templates_dir, temp_dir.path());
    }

    #[test]
    fn test_list_templates_empty() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        
        let result = manager.list_templates();
        assert!(result.is_ok());
        
        let templates = result.unwrap();
        assert!(templates.is_empty());
    }

    #[test]
    fn test_list_templates_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        fs::create_dir_all(&templates_dir).unwrap();
        
        // Create test template files
        fs::write(templates_dir.join("template1.latex"), "test content").unwrap();
        fs::write(templates_dir.join("template2.html"), "test content").unwrap();
        
        let manager = TemplateManager::new(templates_dir);
        let result = manager.list_templates();
        assert!(result.is_ok());
        
        let mut templates = result.unwrap();
        templates.sort();
        assert_eq!(templates, vec!["template1.latex", "template2.html"]);
    }

    #[test]
    fn test_install_template_success() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        
        // Create source template
        let source_path = temp_dir.path().join("source.latex");
        fs::write(&source_path, "template content").unwrap();
        
        let manager = TemplateManager::new(templates_dir.clone());
        let result = manager.install_template(&source_path);
        assert!(result.is_ok());
        
        // Verify template was installed
        let installed_path = templates_dir.join("source.latex");
        assert!(installed_path.exists());
        
        let content = fs::read_to_string(installed_path).unwrap();
        assert_eq!(content, "template content");
    }

    #[test]
    fn test_install_template_missing_source() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        let missing_path = temp_dir.path().join("missing.latex");
        
        let manager = TemplateManager::new(templates_dir);
        let result = manager.install_template(&missing_path);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            AutoDocError::Build { message } => {
                assert!(message.contains("Template file not found"));
            }
            _ => panic!("Expected Build error"),
        }
    }

    #[test]
    fn test_ensure_templates_dir() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("new_templates");
        
        assert!(!templates_dir.exists());
        
        let manager = TemplateManager::new(templates_dir.clone());
        let result = manager.ensure_templates_dir();
        assert!(result.is_ok());
        
        assert!(templates_dir.exists());
        assert!(templates_dir.is_dir());
    }
}
