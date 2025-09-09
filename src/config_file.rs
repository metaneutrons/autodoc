use crate::config::{DocumentMetadata, ProjectConfig};
use crate::errors::{AutoDocError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDocConfig {
    pub project: ProjectSettings,
    pub build: BuildSettings,
    pub templates: TemplateSettings,
    pub metadata: Option<DocumentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
    pub output_dir: Option<PathBuf>,
    pub source_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSettings {
    pub default_format: Option<String>,
    pub watch: Option<bool>,
    pub clean_before_build: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSettings {
    pub pdf_template: Option<String>,
    pub html_template: Option<String>,
    pub docx_template: Option<String>,
}

impl Default for AutoDocConfig {
    fn default() -> Self {
        Self {
            project: ProjectSettings {
                name: "document".to_string(),
                output_dir: Some(PathBuf::from("output")),
                source_dir: None,
            },
            build: BuildSettings {
                default_format: Some("pdf".to_string()),
                watch: Some(false),
                clean_before_build: Some(false),
            },
            templates: TemplateSettings {
                pdf_template: None,
                html_template: None,
                docx_template: None,
            },
            metadata: None,
        }
    }
}

impl AutoDocConfig {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            debug!("Config file not found: {}", path.display());
            return Ok(Self::default());
        }

        info!("Loading config from: {}", path.display());

        let content = fs::read_to_string(path)?;
        let config: AutoDocConfig =
            serde_yaml::from_str(&content).map_err(|e| AutoDocError::Config {
                message: format!("Invalid config file: {}", e),
            })?;

        Ok(config)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        info!("Saving config to: {}", path.display());

        let content = serde_yaml::to_string(self).map_err(|e| AutoDocError::Config {
            message: format!("Failed to serialize config: {}", e),
        })?;

        fs::write(path, content)?;
        Ok(())
    }

    pub fn find_config_file() -> Option<PathBuf> {
        let candidates = [
            "autodoc.yml",
            "autodoc.yaml",
            ".autodoc.yml",
            ".autodoc.yaml",
        ];

        for candidate in &candidates {
            let path = PathBuf::from(candidate);
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn to_project_config(&self) -> ProjectConfig {
        let mut config = ProjectConfig {
            name: self.project.name.clone(),
            ..Default::default()
        };

        if let Some(output_dir) = &self.project.output_dir {
            config.output_dir = output_dir.clone();
        }

        config
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Helper to ensure directory is restored even if test panics
    struct DirectoryGuard {
        original: PathBuf,
    }

    impl DirectoryGuard {
        fn new(new_dir: &Path) -> std::io::Result<Self> {
            let original = std::env::current_dir()?;
            std::env::set_current_dir(new_dir)?;
            Ok(Self { original })
        }
    }

    impl Drop for DirectoryGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.original);
        }
    }

    #[test]
    fn test_autodoc_config_default() {
        let config = AutoDocConfig::default();

        assert_eq!(config.project.name, "document");
        assert_eq!(config.project.output_dir, Some(PathBuf::from("output")));
        assert_eq!(config.build.default_format, Some("pdf".to_string()));
        assert_eq!(config.build.watch, Some(false));
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.yml");

        let original_config = AutoDocConfig::default();
        let save_result = original_config.save_to_file(&config_path);
        assert!(save_result.is_ok());

        assert!(config_path.exists());

        let loaded_result = AutoDocConfig::load_from_file(&config_path);
        assert!(loaded_result.is_ok());

        let loaded_config = loaded_result.unwrap();
        assert_eq!(loaded_config.project.name, original_config.project.name);
        assert_eq!(
            loaded_config.build.default_format,
            original_config.build.default_format
        );
    }

    #[test]
    fn test_config_load_missing_file() {
        let temp_dir = TempDir::new().unwrap();
        let missing_path = temp_dir.path().join("missing.yml");

        let result = AutoDocConfig::load_from_file(&missing_path);
        assert!(result.is_ok());

        // Should return default config when file doesn't exist
        let config = result.unwrap();
        assert_eq!(config.project.name, "document");
    }

    #[test]
    fn test_config_load_invalid_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid.yml");

        fs::write(&config_path, "invalid: yaml: content: [").unwrap();

        let result = AutoDocConfig::load_from_file(&config_path);
        assert!(result.is_err());

        match result.unwrap_err() {
            AutoDocError::Config { message } => {
                assert!(message.contains("Invalid config file"));
            }
            _ => panic!("Expected Config error"),
        }
    }

    #[test]
    fn test_find_config_file_none() {
        let temp_dir = TempDir::new().unwrap();
        let _guard = DirectoryGuard::new(temp_dir.path()).unwrap();

        let result = AutoDocConfig::find_config_file();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_config_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("autodoc.yml");
        fs::write(&config_path, "project:\n  name: test").unwrap();

        let _guard = DirectoryGuard::new(temp_dir.path()).unwrap();

        let result = AutoDocConfig::find_config_file();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), PathBuf::from("autodoc.yml"));
    }

    #[test]
    fn test_to_project_config() {
        let mut config = AutoDocConfig::default();
        config.project.name = "test-project".to_string();
        config.project.output_dir = Some(PathBuf::from("custom-output"));

        let project_config = config.to_project_config();

        assert_eq!(project_config.name, "test-project");
        assert_eq!(project_config.output_dir, PathBuf::from("custom-output"));
    }
}
