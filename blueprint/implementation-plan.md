# docPilot Implementation Plan

## Overview
This plan breaks down the docPilot implementation into manageable phases, each building upon the previous one. The goal is to have a working MVP quickly, then iteratively add advanced features.

## Phase 1: Foundation & MVP (2-3 weeks)

### Step 1.1: Project Setup (2-3 days)
**Goal**: Basic Rust project with CLI framework

**Tasks**:
- [ ] Initialize Cargo project with proper structure
- [ ] Set up basic dependencies (clap, tokio, anyhow, serde)
- [ ] Create module structure (`src/config/`, `src/discovery/`, `src/builders/`)
- [ ] Implement basic CLI with `clap` derive macros
- [ ] Set up logging with `tracing` and `tracing-subscriber`
- [ ] Create basic error types with `thiserror`

**Deliverable**: `docpilot --help` works and shows command structure

### Step 1.2: File Discovery (3-4 days)
**Goal**: Discover and parse markdown files with metadata

**Tasks**:
- [ ] Implement `FileDiscovery` struct with markdown file detection
- [ ] Create `MetadataParser` for YAML frontmatter extraction
- [ ] Add natural sorting for numbered files (00-setup.md, 01-intro.md)
- [ ] Implement file exclusion patterns (README.md, custom patterns)
- [ ] Add dependency extraction from markdown (images, links)
- [ ] Create `DiscoveredFiles` data structure

**Deliverable**: `docpilot status` shows discovered files and metadata

### Step 1.3: Basic PDF Builder (4-5 days)
**Goal**: Generate PDF using pandoc with basic options

**Tasks**:
- [ ] Implement `PdfBuilder` with pandoc command execution
- [ ] Add metadata-to-pandoc-args conversion
- [ ] Implement language detection and babel-lang mapping
- [ ] Add template detection and usage
- [ ] Create output directory management
- [ ] Add basic error handling for pandoc failures

**Deliverable**: `docpilot build pdf` generates working PDF

### Step 1.4: Dependency Validation (2-3 days)
**Goal**: Check and validate external dependencies

**Tasks**:
- [ ] Implement `DependencyChecker` with pandoc/xelatex detection
- [ ] Add version checking for dependencies
- [ ] Create platform-specific installation hints
- [ ] Add dependency validation before builds
- [ ] Implement `check` command with detailed status

**Deliverable**: `docpilot check` validates all dependencies

### Step 1.5: Project Initialization (3-4 days)
**Goal**: Initialize new projects with templates

**Tasks**:
- [ ] Implement `ProjectInitializer` with directory creation
- [ ] Create 00-setup.md template generation
- [ ] Add sample content generation for different project types
- [ ] Implement Eisvogel template download
- [ ] Add git repository initialization (optional)

**Deliverable**: `docpilot init` creates complete project structure

**Phase 1 Milestone**: Working MVP that can initialize projects and build PDFs

## Phase 2: Core Features (3-4 weeks)

### Step 2.1: Multiple Output Formats (3-4 days)
**Goal**: Support DOCX and HTML generation

**Tasks**:
- [ ] Implement `DocxBuilder` with pandoc integration
- [ ] Implement `HtmlBuilder` with self-contained assets
- [ ] Add format-specific argument handling
- [ ] Create `all` command for multiple formats
- [ ] Add format validation and error handling

**Deliverable**: `docpilot build docx` and `docpilot build html` work

### Step 2.2: Native Diagram Generation (4-5 days)
**Goal**: Replace Mermaid CLI with native Rust

**Tasks**:
- [ ] Integrate `pisnge` crate for Mermaid rendering
- [ ] Implement `DiagramGenerator` with .mmd file processing
- [ ] Add SVG to PDF conversion with `svg2pdf` crate
- [ ] Create diagram dependency tracking
- [ ] Add inline mermaid detection in markdown
- [ ] Implement `diagrams` command

**Deliverable**: Mermaid diagrams work without Node.js dependencies

### Step 2.3: Template Management (3-4 days)
**Goal**: Advanced template handling

**Tasks**:
- [ ] Implement `TemplateManager` with template discovery
- [ ] Add template metadata parsing (version, description)
- [ ] Create template installation from files/URLs
- [ ] Implement `templates` command with list/install/remove
- [ ] Add custom template validation

**Deliverable**: `docpilot templates` manages templates effectively

### Step 2.4: File Watching & Auto-rebuild (4-5 days)
**Goal**: Development workflow with auto-rebuild

**Tasks**:
- [ ] Integrate `notify` crate for file system watching
- [ ] Implement `FileWatcher` with selective rebuild triggers
- [ ] Add debouncing to prevent excessive rebuilds
- [ ] Create `watch` command with status updates
- [ ] Add build-and-open functionality

**Deliverable**: `docpilot build --watch` auto-rebuilds on changes

### Step 2.5: Configuration System (3-4 days)
**Goal**: Advanced configuration management

**Tasks**:
- [ ] Create `DocPilotConfig` with TOML support
- [ ] Implement configuration file discovery (docpilot.toml, .docpilot/)
- [ ] Add configuration validation with schemas
- [ ] Create configuration merging (CLI > file > defaults)
- [ ] Implement `config` command for management

**Deliverable**: Flexible configuration system with validation

**Phase 2 Milestone**: Feature-complete tool matching Makefile functionality

## Phase 3: Advanced Features (2-3 weeks)

### Step 3.1: Incremental Builds & Caching (4-5 days)
**Goal**: Performance optimization with smart rebuilds

**Tasks**:
- [ ] Implement `BuildCache` with file hash tracking
- [ ] Add dependency graph for incremental builds
- [ ] Create cache invalidation logic
- [ ] Add build metrics and timing
- [ ] Implement `--no-cache` flag

**Deliverable**: Significantly faster rebuilds for large projects

### Step 3.2: Plugin System (5-6 days)
**Goal**: Extensibility through plugins

**Tasks**:
- [ ] Design `docPilotPlugin` trait with lifecycle hooks
- [ ] Implement `PluginManager` with registration
- [ ] Add plugin discovery and loading
- [ ] Create plugin configuration system
- [ ] Implement `plugin` command for management
- [ ] Create example plugins (custom variables, transformations)

**Deliverable**: Working plugin system with examples

### Step 3.3: Advanced Error Handling (3-4 days)
**Goal**: Professional error reporting and diagnostics

**Tasks**:
- [ ] Create comprehensive `DocPilotError` types
- [ ] Implement `DiagnosticReporter` with rich context
- [ ] Add error recovery and suggestions
- [ ] Create debug mode with verbose output
- [ ] Add error reporting to external services (optional)

**Deliverable**: Professional error messages with helpful suggestions

### Step 3.4: Performance Monitoring (2-3 days)
**Goal**: Build metrics and performance insights

**Tasks**:
- [ ] Implement `PerformanceMonitor` with phase timing
- [ ] Add `BuildMetrics` collection and reporting
- [ ] Create performance profiling mode
- [ ] Add memory usage tracking
- [ ] Implement performance regression detection

**Deliverable**: Detailed build performance insights

**Phase 3 Milestone**: Tool with advanced features

## Phase 4: Polish & Distribution (1-2 weeks)

### Step 4.1: Self-Update System (3-4 days)
**Goal**: Automatic updates and version management

**Tasks**:
- [ ] Implement `SelfUpdater` with GitHub releases integration
- [ ] Add version checking and update notifications
- [ ] Create backup and rollback functionality
- [ ] Add update channels (stable, beta, nightly)
- [ ] Implement `update` command

**Deliverable**: Self-updating capability

### Step 4.2: Shell Completions & Documentation (2-3 days)
**Goal**: Professional CLI experience

**Tasks**:
- [ ] Generate shell completions for bash, zsh, fish
- [ ] Create comprehensive man pages
- [ ] Add `completions` command
- [ ] Write user documentation and tutorials
- [ ] Create developer documentation

**Deliverable**: Professional CLI with completions and docs

### Step 4.3: Testing & Quality Assurance (3-4 days)
**Goal**: Comprehensive test coverage

**Tasks**:
- [ ] Create integration test suite with temporary projects
- [ ] Add unit tests for all core modules
- [ ] Implement property-based testing for metadata parsing
- [ ] Add performance benchmarks
- [ ] Create CI/CD pipeline with automated testing

**Deliverable**: High-quality, well-tested codebase

### Step 4.4: Distribution & Packaging (2-3 days)
**Goal**: Easy installation and distribution

**Tasks**:
- [ ] Create GitHub releases with binaries
- [ ] Publish to crates.io
- [ ] Create Homebrew formula
- [ ] Add Debian/Ubuntu packages
- [ ] Create Docker image
- [ ] Write installation documentation

**Deliverable**: Multiple installation methods available

**Phase 4 Milestone**: Production-ready tool with professional distribution

## Implementation Guidelines

### Development Principles
1. **Test-Driven Development**: Write tests before implementation
2. **Incremental Progress**: Each step should produce working functionality
3. **Documentation First**: Document APIs before implementing
4. **Error Handling**: Comprehensive error handling from day one
5. **Performance Awareness**: Profile and optimize throughout development

### Quality Gates
- [ ] All tests pass
- [ ] Code coverage > 80%
- [ ] No clippy warnings
- [ ] Documentation complete
- [ ] Performance benchmarks meet targets

### Risk Mitigation
- **External Dependencies**: Have fallback plans for pandoc/xelatex issues
- **Platform Compatibility**: Test on Windows, macOS, Linux throughout
- **Performance**: Regular benchmarking to catch regressions
- **User Experience**: Regular user testing and feedback collection

## Success Metrics

### Phase 1 Success
- [ ] Can initialize and build basic PDF projects
- [ ] Installation process is straightforward
- [ ] Basic error handling works correctly

### Phase 2 Success
- [ ] Feature parity with existing Makefile
- [ ] No Node.js dependencies required
- [ ] Development workflow is smooth

### Phase 3 Success
- [ ] Significantly faster than Makefile approach
- [ ] Plugin system enables customization
- [ ] Error messages are helpful and actionable

### Phase 4 Success
- [ ] Professional-grade tool ready for production use
- [ ] Easy installation across platforms
- [ ] Comprehensive documentation and support

## Timeline Summary

- **Phase 1**: 2-3 weeks (MVP)
- **Phase 2**: 3-4 weeks (Feature Complete)
- **Phase 3**: 2-3 weeks (Advanced Features)
- **Phase 4**: 1-2 weeks (Polish & Distribution)

**Total**: 8-12 weeks for complete implementation

This plan provides a clear roadmap from MVP to production-ready tool, with each phase building meaningful value while maintaining momentum toward the final goal.
