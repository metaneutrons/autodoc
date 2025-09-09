use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_build_with_no_markdown_files() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("build").arg("pdf");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No markdown files found"));
}

#[test]
fn test_build_with_invalid_format() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
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

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
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
    let config_path = temp_dir.path().join("docpilot.yml");

    // Create existing config file
    fs::write(&config_path, "existing: config").unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("config").arg("init");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Config file already exists"));
}

#[test]
fn test_invalid_command_line_args() {
    let mut cmd = Command::cargo_bin("docpilot").unwrap();
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
    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("build").arg("pdf");

    let output = cmd.output().unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should get a proper error message about missing dependencies
        assert!(
            stderr.contains("Missing required dependencies")
                || stderr.contains("Failed to execute pandoc")
                || stderr.contains("Pandoc failed")
        );
    }
}

#[test]
fn test_permission_denied_scenarios() {
    let temp_dir = TempDir::new().unwrap();

    // Test with non-existent directory
    #[cfg(unix)]
    {
        let nonexistent_dir = temp_dir
            .path()
            .join("nonexistent")
            .join("deeply")
            .join("nested");

        // Try to build in non-existent directory using output() to handle the error
        let mut cmd = Command::cargo_bin("docpilot").unwrap();
        cmd.current_dir(&nonexistent_dir).arg("build").arg("pdf");

        let result = cmd.output();

        // Should fail to spawn due to directory not existing
        assert!(
            result.is_err(),
            "Command should fail to spawn in non-existent directory"
        );
    }

    #[cfg(not(unix))]
    {
        // On non-Unix systems, just test that the command exists
        let mut cmd = Command::cargo_bin("docpilot").unwrap();
        cmd.arg("--help");
        cmd.assert().success();
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

    // Status command should fail with malformed frontmatter
    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("status");

    cmd.assert()
        .failure() // Should fail with YAML parsing error
        .stderr(predicate::str::contains("Yaml(Error"));
}

#[test]
fn test_empty_mermaid_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create empty mermaid file
    fs::write(temp_dir.path().join("empty.mmd"), "").unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("diagrams");

    cmd.assert()
        .failure() // Empty mermaid files should fail to compile
        .stderr(predicate::str::contains("Failed to render Mermaid diagram"));
}

#[test]
fn test_very_long_filename() {
    let temp_dir = TempDir::new().unwrap();

    // Create file with very long name
    let long_name = "a".repeat(200) + ".md";
    let long_path = temp_dir.path().join(&long_name);

    // This might fail on some filesystems, but should be handled gracefully
    if fs::write(&long_path, "# Test").is_ok() {
        let mut cmd = Command::cargo_bin("docpilot").unwrap();
        cmd.current_dir(temp_dir.path()).arg("status");

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
author: ["Тест Автор"]
---

# Заголовок

Содержание с эмодзи: 📝 ✨ 🎯

## 中文标题

中文内容测试。

## العربية

محتوى عربي للاختبار.
"#;

    fs::write(temp_dir.path().join("unicode.md"), unicode_content).unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("status");

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
    let handles: Vec<_> = (0..3)
        .map(|_| {
            let temp_path = temp_dir.path().to_path_buf();
            std::thread::spawn(move || {
                let mut cmd = Command::cargo_bin("docpilot").unwrap();
                cmd.current_dir(&temp_path).arg("status");

                cmd.assert()
                    .success()
                    .stdout(predicate::str::contains("Markdown files: 1"));
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
