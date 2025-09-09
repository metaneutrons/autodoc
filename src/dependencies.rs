use crate::errors::{DocPilotError, Result};
use std::process::Command;
use tracing::{debug, info};
use which::which;

#[derive(Debug, Clone)]
pub struct DependencyStatus {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
    pub required: bool,
    pub install_hint: Option<String>,
}

pub struct DependencyChecker;

impl DependencyChecker {
    pub fn check_all() -> Result<Vec<DependencyStatus>> {
        let deps = vec![Self::check_pandoc()?, Self::check_xelatex()?];

        Ok(deps)
    }

    pub fn validate_for_build(format: &str) -> Result<()> {
        info!("Validating dependencies for {} build", format);

        let deps = Self::check_all()?;
        let mut missing_required = Vec::new();

        for dep in deps {
            let required_for_format = match format {
                "pdf" | "all" => dep.name == "pandoc" || dep.name == "xelatex",
                "docx" | "html" => dep.name == "pandoc",
                _ => dep.required,
            };

            if required_for_format && !dep.available {
                missing_required.push(dep);
            }
        }

        if !missing_required.is_empty() {
            let mut error_msg = format!("Missing required dependencies for {} format:\n", format);
            for dep in missing_required {
                error_msg.push_str(&format!(
                    "  - {}: {}\n",
                    dep.name,
                    dep.install_hint
                        .unwrap_or_else(|| "Install manually".to_string())
                ));
            }

            return Err(DocPilotError::Dependency {
                tool: "multiple".to_string(),
                hint: error_msg,
            });
        }

        Ok(())
    }

    fn check_pandoc() -> Result<DependencyStatus> {
        let available = which("pandoc").is_ok();
        let version = if available {
            Self::get_command_version("pandoc", &["--version"])?
        } else {
            None
        };

        Ok(DependencyStatus {
            name: "pandoc".to_string(),
            available,
            version,
            required: true,
            install_hint: Some(Self::get_install_hint("pandoc")),
        })
    }

    fn check_xelatex() -> Result<DependencyStatus> {
        let available = which("xelatex").is_ok();
        let version = if available {
            Self::get_command_version("xelatex", &["--version"])?
        } else {
            None
        };

        Ok(DependencyStatus {
            name: "xelatex".to_string(),
            available,
            version,
            required: true,
            install_hint: Some(Self::get_install_hint("texlive")),
        })
    }

    fn get_command_version(cmd: &str, args: &[&str]) -> Result<Option<String>> {
        debug!("Checking version for command: {}", cmd);

        let output = Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| DocPilotError::Build {
                message: format!("Failed to execute {}: {}", cmd, e),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Extract version from first line
            if let Some(first_line) = stdout.lines().next() {
                return Ok(Some(first_line.to_string()));
            }
        }

        Ok(None)
    }

    fn get_install_hint(package: &str) -> String {
        if cfg!(target_os = "macos") {
            match package {
                "pandoc" => "brew install pandoc".to_string(),
                "texlive" => "brew install --cask mactex".to_string(),
                _ => format!("brew install {}", package),
            }
        } else if cfg!(target_os = "linux") {
            match package {
                "pandoc" => "sudo apt install pandoc".to_string(),
                "texlive" => "sudo apt install texlive-xetex".to_string(),
                _ => format!("sudo apt install {}", package),
            }
        } else {
            format!("Install {} via your package manager", package)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_status_creation() {
        let dep = DependencyStatus {
            name: "test".to_string(),
            available: true,
            version: Some("1.0.0".to_string()),
            required: true,
            install_hint: Some("Install with: brew install test".to_string()),
        };

        assert_eq!(dep.name, "test");
        assert!(dep.available);
        assert_eq!(dep.version, Some("1.0.0".to_string()));
        assert!(dep.required);
    }

    #[test]
    fn test_check_all_returns_dependencies() {
        let result = DependencyChecker::check_all();
        assert!(result.is_ok());

        let deps = result.unwrap();
        assert!(!deps.is_empty());

        // Should have at least pandoc and xelatex
        let names: Vec<&str> = deps.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"pandoc"));
        assert!(names.contains(&"xelatex"));
    }

    #[test]
    fn test_validate_for_build_pdf() {
        // This test will pass if pandoc is available, otherwise will test error handling
        let result = DependencyChecker::validate_for_build("pdf");

        // Either succeeds or fails with proper error message
        match result {
            Ok(()) => {
                // Dependencies available
            }
            Err(DocPilotError::Dependency { tool: _, hint }) => {
                assert!(hint.contains("Missing required dependencies"));
            }
            Err(_) => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_validate_for_build_docx() {
        let result = DependencyChecker::validate_for_build("docx");

        match result {
            Ok(()) => {
                // Dependencies available
            }
            Err(DocPilotError::Dependency { tool: _, hint }) => {
                assert!(hint.contains("docx format"));
            }
            Err(_) => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_validate_for_build_html() {
        let result = DependencyChecker::validate_for_build("html");

        match result {
            Ok(()) => {
                // Dependencies available
            }
            Err(DocPilotError::Dependency { tool: _, hint }) => {
                assert!(hint.contains("html format"));
            }
            Err(_) => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_validate_for_build_all() {
        let result = DependencyChecker::validate_for_build("all");

        match result {
            Ok(()) => {
                // All dependencies available
            }
            Err(DocPilotError::Dependency { tool: _, hint }) => {
                assert!(hint.contains("all format"));
            }
            Err(_) => panic!("Unexpected error type"),
        }
    }
}
