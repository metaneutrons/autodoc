use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Automatic document generation"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("docpilot"));
}

#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .arg("--name")
        .arg("test-project");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Project initialized"));

    // Verify files were created
    assert!(temp_dir.path().join("00-setup.md").exists());
    assert!(temp_dir.path().join("01-introduction.md").exists());
}

#[test]
fn test_check_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("check");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DEPENDENCY CHECK"));
}

#[test]
fn test_status_command_empty_project() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("PROJECT STATUS"))
        .stdout(predicate::str::contains("Markdown files: 0"));
}

#[test]
fn test_status_command_with_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create test markdown file
    fs::write(temp_dir.path().join("test.md"), "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Markdown files: 1"));
}

#[test]
fn test_config_init_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("config").arg("init");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created config file"));

    // Verify config file was created
    assert!(temp_dir.path().join("docpilot.yml").exists());
}

#[test]
fn test_config_show_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("config").arg("show");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No config file found"));
}

#[test]
fn test_templates_list_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("templates")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available templates"));
}

#[test]
fn test_diagrams_command_no_files() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("diagrams");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No Mermaid files found"));
}

#[test]
fn test_diagrams_command_with_mermaid() {
    let temp_dir = TempDir::new().unwrap();

    // Create test mermaid file
    fs::write(temp_dir.path().join("test.mmd"), "graph TD\n    A --> B").unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("diagrams");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Processing 1 Mermaid diagrams"));
}

#[test]
fn test_clean_command() {
    let temp_dir = TempDir::new().unwrap();

    // Create output directory
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join("test.pdf"), "fake pdf").unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("clean");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Cleaned output directory"));

    // Verify output directory was removed
    assert!(!output_dir.exists());
}

#[test]
fn test_build_command_no_files() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.current_dir(temp_dir.path()).arg("build").arg("pdf");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No markdown files found"));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.arg("invalid-command");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_build_help() {
    let mut cmd = Command::cargo_bin("docpilot").unwrap();
    cmd.arg("build").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Build documents"))
        .stdout(predicate::str::contains("pdf"))
        .stdout(predicate::str::contains("docx"))
        .stdout(predicate::str::contains("html"))
        .stdout(predicate::str::contains("all"));
}
