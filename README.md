# docPilot 🚀

[![CI](https://github.com/metaneutrons/docpilot/workflows/CI/badge.svg)](https://github.com/metaneutrons/docpilot/actions)
[![Release](https://img.shields.io/github/v/release/metaneutrons/docpilot)](https://github.com/metaneutrons/docpilot/releases)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](https://github.com/metaneutrons/docpilot/releases)

> **Automatic document generation tool that transforms Markdown into professional documents**

docPilot is a powerful Rust-based CLI tool that orchestrates Pandoc and XeLaTeX to generate beautiful PDF, DOCX, and HTML documents from Markdown sources. Built for technical writers, researchers, and documentation teams who demand professional output without the complexity.

## ✨ Features

- 🎯 **Multi-Format Output** - PDF, DOCX, HTML with format-specific optimizations
- 🎨 **Professional Templates** - Eisvogel LaTeX template with auto-download
- 📊 **Native Diagrams** - Built-in Mermaid rendering with mermaid-rs (no Node.js required)
- ⚙️ **Smart Configuration** - YAML-based project configuration with auto-discovery
- 🌍 **Multi-Language** - Babel language auto-detection for LaTeX
- 🔍 **Dependency Management** - Smart validation with installation hints
- 📁 **Project Structure** - Organized file discovery with natural sorting
- 🚀 **Zero Node.js** - Pure Rust implementation, no JavaScript dependencies

## 🚀 Quick Start

### Installation

#### Homebrew (macOS/Linux)
```bash
brew install metaneutrons/tap/docpilot
```

#### Download from releases
```bash
curl -L https://github.com/metaneutrons/docpilot/releases/latest/download/docpilot-linux-x86_64.tar.gz -o docpilot.tar.gz
tar xzf docpilot.tar.gz
sudo mv docpilot /usr/local/bin/
```

#### Build from source
```bash
git clone https://github.com/metaneutrons/docpilot.git
cd docpilot
cargo install --path .
```

### Usage

```bash
# Initialize new project
docpilot init my-document

# Build PDF (requires pandoc + xelatex)
docpilot build pdf

# Build all formats
docpilot build all

# Check dependencies
docpilot check

# Project status
docpilot status
```

## 📖 Example

```yaml
---
title: "Technical Report"
author: "Your Name"
date: "2024"
lang: "en"
---

# Introduction

This document demonstrates docPilot's capabilities.

## Features

- Professional PDF generation
- Multi-language support
- Template management
```

```bash
docpilot build pdf
# → Generates professional PDF with Eisvogel template
```

## 🏗️ Architecture

docPilot follows a **smart orchestration** approach:

- **Discovery Engine** - Finds and analyzes Markdown files
- **Metadata Parser** - Extracts YAML frontmatter with validation
- **Template Manager** - Downloads and manages LaTeX templates
- **Build Pipeline** - Orchestrates Pandoc with optimized arguments
- **Dependency Validator** - Ensures required tools are available

## 🛠️ Dependencies

**Required:**
- [Pandoc](https://pandoc.org/) - Document conversion engine

**Optional (for PDF):**
- [XeLaTeX](https://tug.org/xetex/) - LaTeX engine for PDF generation

docPilot provides installation hints for missing dependencies and includes **native Mermaid diagram rendering** built-in.

## 📊 Commands

| Command | Description |
|---------|-------------|
| `init` | Initialize new project with templates |
| `build <format>` | Generate documents (pdf, docx, html, all) |
| `check` | Validate dependencies |
| `status` | Show project overview |
| `templates` | Manage LaTeX templates |
| `config` | Project configuration |
| `diagrams` | Process Mermaid diagrams |
| `clean` | Remove output files |

## 🎯 Configuration

```yaml
# docpilot.yml
project:
  name: "my-document"
  output_dir: "output"

build:
  default_format: "pdf"
  clean_before_build: false

templates:
  pdf_template: "eisvogel"

metadata:
  author: ["Your Name"]
  lang: "en"
```

## 🧪 Development

```bash
# Run tests
cargo test

# Run with pre-commit hooks
./install-hooks.sh

# Check formatting
cargo fmt --check

# Lint code
cargo clippy -- -D warnings
```

## 📈 Metrics

- **8,500+** lines of production Rust code
- **44** automated tests with comprehensive coverage
- **15/15** integration tests passing
- **Cross-platform** support (Linux, macOS, Windows)
- **Zero** Node.js dependencies

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feat/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feat/amazing-feature`)
5. Open Pull Request

We use [Conventional Commits](https://conventionalcommits.org/) with types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `blueprint`.

## 📄 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Pandoc](https://pandoc.org/) - Universal document converter
- [Eisvogel](https://github.com/Wandmalfarbe/pandoc-latex-template) - Beautiful LaTeX template
- [Rust Community](https://www.rust-lang.org/community) - Amazing ecosystem

---

<div align="center">

**[Documentation](https://github.com/metaneutrons/docpilot/wiki) • [Releases](https://github.com/metaneutrons/docpilot/releases) • [Issues](https://github.com/metaneutrons/docpilot/issues)**

Made with ❤️ and 🦀 by [metaneutrons](https://github.com/metaneutrons)

</div>

## 🛠️ Development

### Pre-commit Hook
docPilot includes an automatic formatting pre-commit hook:
- **Auto-formats** Rust code with `cargo fmt`
- **Runs Clippy linter** with `-D warnings` (treats warnings as errors)
- **Prevents CI failures** by fixing issues locally
- **Validates** commit message format

The hook is automatically installed in `.git/hooks/pre-commit`.
