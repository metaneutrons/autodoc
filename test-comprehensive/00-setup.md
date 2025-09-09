---
# Document Metadata
title: "test-comprehensive"
author: ["Your Name"]
date: "2025-09-09"
# subtitle: "Document Subtitle"

# Language and Localization
lang: "en"
# babel-lang: "ngerman"

# Document Structure
top-level-division: "section"
numbersections: true
# secnumdepth: 3
# toc: true
# toc-depth: 3
# lof: true
# lot: true

# Document Class and Layout
# documentclass: "article"
# classoption: ["11pt", "a4paper"]
# geometry: ["margin=2.5cm"]
# fontsize: "11pt"
# linestretch: 1.2

# Fonts (requires XeLaTeX)
# mainfont: "Times New Roman"
# sansfont: "Arial"
# monofont: "Courier New"
# mathfont: "Latin Modern Math"

# Headers and Footers
# header-left: "Document Title"
# header-center: ""
# header-right: "\\today"
# footer-left: "Author Name"
# footer-center: ""
# footer-right: "\\thepage"

# Bibliography and Citations
# bibliography: "references.bib"
# csl: "ieee.csl"
# link-citations: true
# reference-section-title: "References"

# Code Highlighting
# highlight-style: "github"
# listings: true

# Links and Cross-references
# linkcolor: "blue"
# urlcolor: "blue"
# citecolor: "blue"

# PDF-specific Options
# colorlinks: true
# bookmarks: true
# bookmarksnumbered: true
# pdfcreator: "AutoDoc"
# pdfproducer: "Pandoc with XeLaTeX"

# Eisvogel Template Options
# titlepage: true
# titlepage-color: "FFFFFF"
# titlepage-text-color: "000000"
# titlepage-rule-color: "000000"
# titlepage-rule-height: 2
# logo: "images/logo.png"
# logo-width: "100"
# disable-header-and-footer: false

# Table Options
# table-use-row-colors: true

# Figure Options
# fig-caption-location: "bottom"
# tbl-caption-location: "top"

# Custom Variables
# company: "Your Company"
# department: "Your Department"
# version: "1.0"
# status: "Draft"
---

# Project Setup

This document serves as the main configuration file for your AutoDoc project.
All document settings are defined in the YAML frontmatter above.

## Configuration Guide

### Essential Settings
- **title**: Document title (appears on title page)
- **author**: List of authors (use array format)
- **date**: Document date (use YYYY-MM-DD format)
- **lang**: Document language (en, de, fr, es, etc.)

### Layout Options
- **geometry**: Page margins and layout
- **fontsize**: Base font size (10pt, 11pt, 12pt)
- **numbersections**: Enable section numbering
- **toc**: Enable table of contents

### Advanced Features
- **bibliography**: Reference file for citations
- **highlight-style**: Code syntax highlighting theme
- **titlepage**: Enable custom title page (Eisvogel)

## Next Steps

1. Customize the frontmatter above for your document
2. Add content in numbered markdown files
3. Place images in the images/ directory
4. Run autodoc build pdf to generate your document

For more information, visit: https://github.com/metaneutrons/autodoc
