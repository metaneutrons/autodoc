use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_complete_workflow_init_to_build() {
    let temp_dir = TempDir::new().unwrap();

    // Step 1: Initialize project
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .arg("--name")
        .arg("e2e-test");

    cmd.assert().success();

    // Verify initialization
    assert!(temp_dir.path().join("00-setup.md").exists());
    assert!(temp_dir.path().join("01-introduction.md").exists());

    // Step 2: Check status
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Markdown files: 2"));

    // Step 3: Check dependencies
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("check");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DEPENDENCY CHECK"));

    // Step 4: Try to build (may fail if pandoc not available, but should handle gracefully)
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("build").arg("html"); // HTML is most likely to work without XeLaTeX

    // Either succeeds or fails with proper error message
    let output = cmd.output().unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Missing required dependencies") || stderr.contains("Pandoc failed")
        );
    }
}

#[test]
fn test_config_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Step 1: Create config
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("config").arg("init");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created config file"));

    // Step 2: Show config
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("config").arg("show");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Configuration from: autodoc.yml"))
        .stdout(predicate::str::contains("name: document"));

    // Step 3: Verify config file content
    let config_content = fs::read_to_string(temp_dir.path().join("autodoc.yml")).unwrap();
    assert!(config_content.contains("project:"));
    assert!(config_content.contains("build:"));
    assert!(config_content.contains("templates:"));
}

#[test]
fn test_template_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Step 1: List templates (should be empty)
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("templates")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No templates installed"));

    // Step 2: Create a test template
    let templates_dir = temp_dir.path().join("templates");
    fs::create_dir_all(&templates_dir).unwrap();
    fs::write(templates_dir.join("test.latex"), "\\documentclass{article}").unwrap();

    // Step 3: List templates again (should show our template)
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("templates")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test.latex"));

    // Step 4: Install another template
    let source_template = temp_dir.path().join("source.latex");
    fs::write(&source_template, "\\documentclass{report}").unwrap();

    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("templates")
        .arg("install")
        .arg(source_template.to_str().unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Template installed successfully"));

    // Verify template was installed
    assert!(templates_dir.join("source.latex").exists());
}

#[test]
fn test_diagram_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Step 1: Check diagrams with no files
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("diagrams");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No Mermaid files found"));

    // Step 2: Create mermaid files
    fs::write(
        temp_dir.path().join("diagram1.mmd"),
        "graph TD\n    A --> B",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("diagram2.mmd"),
        "sequenceDiagram\n    A->>B: Hello",
    )
    .unwrap();

    // Step 3: Process diagrams
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("diagrams");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Processing 2 Mermaid diagrams"))
        .stdout(predicate::str::contains("Diagram processing complete"));

    // Step 4: Verify output directory was created
    assert!(temp_dir.path().join("output/diagrams").exists());

    // Step 5: Check status to see diagrams are detected
    let mut cmd = Command::cargo_bin("autodoc").unwrap();
    cmd.current_dir(temp_dir.path()).arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Mermaid files:  2"));
}

#[test]
fn test_full_project_lifecycle() {
    let temp_dir = TempDir::new().unwrap();

    // 1. Initialize project
    Command::cargo_bin("autodoc")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--name")
        .arg("lifecycle-test")
        .assert()
        .success();

    // 2. Create config
    Command::cargo_bin("autodoc")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("config")
        .arg("init")
        .assert()
        .success();

    // 3. Add custom content
    fs::write(
        temp_dir.path().join("custom.md"),
        r#"---
title: "Custom Document"
author: "Test Author"
---

# Custom Content

This is a test document with custom content.
"#,
    )
    .unwrap();

    // 4. Add mermaid diagram
    fs::write(
        temp_dir.path().join("flow.mmd"),
        "graph LR\n    Start --> End",
    )
    .unwrap();

    // 5. Check final status
    Command::cargo_bin("autodoc")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Markdown files: 3"))
        .stdout(predicate::str::contains("Mermaid files:  1"));

    // 6. Process diagrams
    Command::cargo_bin("autodoc")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("diagrams")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processing 1 Mermaid diagrams"));

    // 7. Clean up
    Command::cargo_bin("autodoc")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("clean")
        .assert()
        .success();

    // 8. Verify cleanup
    assert!(!temp_dir.path().join("output").exists());
}
