# AutoDoc 🚀

[![CI](https://github.com/metaneutrons/autodoc/workflows/CI/badge.svg)](https://github.com/metaneutrons/autodoc/actions)
[![Release](https://img.shields.io/github/v/release/metaneutrons/autodoc)](https://github.com/metaneutrons/autodoc/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](https://github.com/metaneutrons/autodoc/releases)

> **Automatic document generation tool that transforms Markdown into professional documents**

AutoDoc is a powerful Rust-based CLI tool that orchestrates Pandoc and XeLaTeX to generate beautiful PDF, DOCX, and HTML documents from Markdown sources. Built for technical writers, researchers, and documentation teams who demand professional output without the complexity.

## ✨ Features

- 🎯 **Multi-Format Output** - PDF, DOCX, HTML with format-specific optimizations
- 🎨 **Professional Templates** - Eisvogel LaTeX template with auto-download
- 📊 **Native Diagrams** - Mermaid diagram processing with CLI integration
- ⚙️ **Smart Configuration** - YAML-based project configuration with auto-discovery
- 🌍 **Multi-Language** - Babel language auto-detection for LaTeX
- 🔍 **Dependency Management** - Smart validation with installation hints
- 📁 **Project Structure** - Organized file discovery with natural sorting
- 🚀 **Zero Node.js** - Pure Rust implementation, no JavaScript dependencies

## 🚀 Quick Start

### Installation

```bash
# Download from releases
curl -L https://github.com/metaneutrons/autodoc/releases/latest/download/autodoc-linux-x86_64 -o autodoc
chmod +x autodoc

# Or build from source
git clone https://github.com/metaneutrons/autodoc.git
cd autodoc
cargo install --path .
```

### Usage

```bash
# Initialize new project
autodoc init my-document

# Build PDF (requires pandoc + xelatex)
autodoc build pdf

# Build all formats
autodoc build all

# Check dependencies
autodoc check

# Project status
autodoc status
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

This document demonstrates AutoDoc's capabilities.

## Features

- Professional PDF generation
- Multi-language support
- Template management
```

```bash
autodoc build pdf
# → Generates professional PDF with Eisvogel template
```

## 🏗️ Architecture

AutoDoc follows a **smart orchestration** approach:

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

**Optional (for Diagrams):**
- [Mermaid CLI](https://github.com/mermaid-js/mermaid-cli) - Diagram rendering

AutoDoc provides installation hints for missing dependencies.

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
# autodoc.yml
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

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Pandoc](https://pandoc.org/) - Universal document converter
- [Eisvogel](https://github.com/Wandmalfarbe/pandoc-latex-template) - Beautiful LaTeX template
- [Rust Community](https://www.rust-lang.org/community) - Amazing ecosystem

---

<div align="center">

**[Documentation](https://github.com/metaneutrons/autodoc/wiki) • [Releases](https://github.com/metaneutrons/autodoc/releases) • [Issues](https://github.com/metaneutrons/autodoc/issues)**

Made with ❤️ and 🦀 by [metaneutrons](https://github.com/metaneutrons)

</div>
