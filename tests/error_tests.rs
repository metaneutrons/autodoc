use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_build_with_no_markdown_files() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("build")
        .arg("pdf");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No markdown files found"));
}

#[test]
fn test_build_with_invalid_format() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("build")
        .arg("invalid-format");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_template_install_missing_file() {
    let temp_dir = TempDir::new().unwrap();
    let missing_file = temp_dir.path().join("missing.latex");
    
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("templates")
        .arg("install")
        .arg(missing_file.to_str().unwrap());
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Template file not found"));
}

#[test]
fn test_config_init_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("autodoc.yml");
    
    // Create existing config file
    fs::write(&config_path, "existing: config").unwrap();
    
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("config")
        .arg("init");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Config file already exists"));
}

#[test]
fn test_invalid_command_line_args() {
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.arg("--invalid-flag");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
fn test_build_missing_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create markdown file
    fs::write(temp_dir.path().join("test.md"), "# Test").unwrap();
    
    // Try to build (will likely fail due to missing pandoc in test environment)
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("build")
        .arg("pdf");
    
    let output = cmd.output().unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should get a proper error message about missing dependencies
        assert!(
            stderr.contains("Missing required dependencies") || 
            stderr.contains("Failed to execute pandoc") ||
            stderr.contains("Pandoc failed")
        );
    }
}

#[test]
fn test_permission_denied_scenarios() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a directory we can't write to (on Unix systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        let readonly_dir = temp_dir.path().join("readonly");
        fs::create_dir(&readonly_dir).unwrap();
        
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_mode(0o444); // Read-only
        fs::set_permissions(&readonly_dir, perms).unwrap();
        
        // Try to initialize in readonly directory
        let mut cmd = Command::cargo_bin("autodoc").unwrap();
        cmd.current_dir(&readonly_dir)
            .arg("init");
        
        let output = cmd.output().unwrap();
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains("Permission denied") || stderr.contains("Error"));
        }
        
        // Restore permissions for cleanup
        let mut restore_perms = fs::metadata(&readonly_dir).unwrap().permissions();
        restore_perms.set_mode(0o755);
        fs::set_permissions(&readonly_dir, restore_perms).unwrap();
    }
}

#[test]
fn test_malformed_yaml_frontmatter() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create markdown file with malformed YAML frontmatter
    let malformed_content = r#"---
title: "Test Document
author: [Missing closing bracket
date: 2024-01-01
---

# Content"#;
    
    fs::write(temp_dir.path().join("malformed.md"), malformed_content).unwrap();
    
    // Status command should handle malformed frontmatter gracefully
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("status");
    
    cmd.assert()
        .success() // Should not crash
        .stdout(predicate::str::contains("Markdown files: 1"));
}

#[test]
fn test_empty_mermaid_file() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create empty mermaid file
    fs::write(temp_dir.path().join("empty.mmd"), "").unwrap();
    
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("diagrams");
    
    cmd.assert()
        .success() // Should handle empty files gracefully
        .stdout(predicate::str::contains("Processing 1 Mermaid diagrams"));
}

#[test]
fn test_very_long_filename() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create file with very long name
    let long_name = "a".repeat(200) + ".md";
    let long_path = temp_dir.path().join(&long_name);
    
    // This might fail on some filesystems, but should be handled gracefully
    if fs::write(&long_path, "# Test").is_ok() {
        let mut cmd = Command::cargo_bin("autodoc").unwrap();
        cmd.current_dir(temp_dir.path())
            .arg("status");
        
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Markdown files: 1"));
    }
}

#[test]
fn test_unicode_content() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create markdown file with Unicode content
    let unicode_content = r#"---
title: "测试文档 🚀"
author: "Тест Автор"
---

# Заголовок

Содержание с эмодзи: 📝 ✨ 🎯

## 中文标题

中文内容测试。

## العربية

محتوى عربي للاختبار.
"#;
    
    fs::write(temp_dir.path().join("unicode.md"), unicode_content).unwrap();
    
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("status");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Markdown files: 1"));
}

#[test]
fn test_concurrent_operations() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test file
    fs::write(temp_dir.path().join("test.md"), "# Test").unwrap();
    
    // Run multiple status commands concurrently (should not interfere)
    let handles: Vec<_> = (0..3).map(|_| {
        let temp_path = temp_dir.path().to_path_buf();
        std::thread::spawn(move || {
            let mut cmd = Command::cargo_bin("autodoc").unwrap();
            cmd.current_dir(&temp_path)
                .arg("status");
            
            cmd.assert()
                .success()
                .stdout(predicate::str::contains("Markdown files: 1"));
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
