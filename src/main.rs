use clap::{Parser, Subcommand};
use tracing::{info, error};
use std::path::PathBuf;
use std::fs;

mod errors;
mod config;
mod config_file;
mod discovery;
mod builders;
mod dependencies;
mod init;
mod diagrams;
mod watcher;
mod templates;

use errors::{AutoDocError, Result};

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    ")"
);

#[derive(Parser)]
#[command(name = "autodoc")]
#[command(about = "Enterprise-grade document generation with Pandoc")]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, global = true)]
    verbose: bool,
    
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
    
    /// Build documents
    Build {
        #[command(subcommand)]
        format: BuildFormat,
    },
    
    /// Check dependencies
    Check,
    
    /// Show project status
    Status,
    
    /// Clean generated files
    Clean,
    
    /// Manage templates
    Templates {
        #[command(subcommand)]
        action: Option<TemplateCommands>,
    },
    
    /// Generate diagrams
    Diagrams,
    
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: Option<ConfigCommands>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Initialize default config file
    Init,
    
    /// Show current configuration
    Show,
}

#[derive(Subcommand)]
enum BuildFormat {
    /// Build PDF output
    Pdf {
        #[arg(long)]
        watch: bool,
    },
    /// Build DOCX output
    Docx {
        #[arg(long)]
        watch: bool,
    },
    /// Build HTML output
    Html {
        #[arg(long)]
        watch: bool,
    },
    /// Build all formats
    All,
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// List available templates
    List,
    
    /// Download Eisvogel template
    DownloadEisvogel,
    
    /// Install custom template
    Install { path: PathBuf },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("autodoc={}", level))
        .init();
    
    info!("AutoDoc starting...");
    
    match cli.command {
        Commands::Init { name } => {
            let project_name = name.unwrap_or_else(|| {
                std::env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            });
            
            info!("Initializing project: {}", project_name);
            
            let initializer = init::ProjectInitializer::new(project_name);
            initializer.initialize().await?;
        }
        
        Commands::Build { format } => {
            let config = config::ProjectConfig::default();
            
            let (format_str, watch) = match &format {
                BuildFormat::Pdf { watch } => ("pdf", *watch),
                BuildFormat::Docx { watch } => ("docx", *watch),
                BuildFormat::Html { watch } => ("html", *watch),
                BuildFormat::All => ("all", false),
            };
            
            if watch {
                let watcher = watcher::FileWatcher::new(config);
                return watcher.watch_and_build(format_str).await;
            }
            
            let discovery = discovery::FileDiscovery::new(config.clone());
            let files = discovery.discover_all()?;
            
            if files.markdown_files.is_empty() {
                error!("No markdown files found");
                return Err(AutoDocError::Build { 
                    message: "No markdown files found in current directory".to_string() 
                });
            }
            
            match format {
                BuildFormat::Pdf { .. } => {
                    dependencies::DependencyChecker::validate_for_build("pdf")?;
                    let builder = builders::PdfBuilder::new(config.clone());
                    builder.ensure_output_dir()?;
                    let output_path = config.output_dir.join(format!("{}.pdf", config.name));
                    builder.build(&files.markdown_files, &output_path).await?;
                    println!("📄 PDF built successfully: {}", output_path.display());
                }
                BuildFormat::Docx { .. } => {
                    dependencies::DependencyChecker::validate_for_build("docx")?;
                    let builder = builders::DocxBuilder::new(config.clone());
                    builder.ensure_output_dir()?;
                    let output_path = config.output_dir.join(format!("{}.docx", config.name));
                    builder.build(&files.markdown_files, &output_path).await?;
                    println!("📄 DOCX built successfully: {}", output_path.display());
                }
                BuildFormat::Html { .. } => {
                    dependencies::DependencyChecker::validate_for_build("html")?;
                    let builder = builders::HtmlBuilder::new(config.clone());
                    builder.ensure_output_dir()?;
                    let output_path = config.output_dir.join(format!("{}.html", config.name));
                    builder.build(&files.markdown_files, &output_path).await?;
                    println!("🌐 HTML built successfully: {}", output_path.display());
                }
                BuildFormat::All => {
                    dependencies::DependencyChecker::validate_for_build("all")?;
                    
                    let pdf_builder = builders::PdfBuilder::new(config.clone());
                    pdf_builder.ensure_output_dir()?;
                    let pdf_path = config.output_dir.join(format!("{}.pdf", config.name));
                    pdf_builder.build(&files.markdown_files, &pdf_path).await?;
                    
                    let docx_builder = builders::DocxBuilder::new(config.clone());
                    let docx_path = config.output_dir.join(format!("{}.docx", config.name));
                    docx_builder.build(&files.markdown_files, &docx_path).await?;
                    
                    let html_builder = builders::HtmlBuilder::new(config.clone());
                    let html_path = config.output_dir.join(format!("{}.html", config.name));
                    html_builder.build(&files.markdown_files, &html_path).await?;
                    
                    println!("📄 All formats built successfully:");
                    println!("  PDF:  {}", pdf_path.display());
                    println!("  DOCX: {}", docx_path.display());
                    println!("  HTML: {}", html_path.display());
                }
            }
        }
        
        Commands::Check => {
            info!("Checking dependencies");
            
            match dependencies::DependencyChecker::check_all() {
                Ok(deps) => {
                    println!("🔍 DEPENDENCY CHECK");
                    println!("===================");
                    
                    let mut all_good = true;
                    
                    for dep in deps {
                        let status_icon = if dep.available { "🟢" } else { "🔴" };
                        let required_text = if dep.required { "REQUIRED" } else { "OPTIONAL" };
                        
                        println!("{} {}: {} ({})", status_icon, dep.name, 
                            if dep.available { "Available" } else { "Missing" }, required_text);
                        
                        if let Some(version) = &dep.version {
                            println!("    Version: {}", version);
                        }
                        
                        if !dep.available {
                            if let Some(hint) = &dep.install_hint {
                                println!("    Install: {}", hint);
                            }
                            if dep.required {
                                all_good = false;
                            }
                        }
                        println!();
                    }
                    
                    if all_good {
                        println!("✅ All required dependencies are available!");
                    } else {
                        println!("❌ Some required dependencies are missing. Install them to continue.");
                    }
                }
                Err(e) => {
                    error!("Failed to check dependencies: {}", e);
                    return Err(e);
                }
            }
        }
        
        Commands::Status => {
            info!("Checking project status");
            
            let config = config::ProjectConfig::default();
            let discovery = discovery::FileDiscovery::new(config.clone());
            
            match discovery.discover_all() {
                Ok(files) => {
                    println!("📊 PROJECT STATUS");
                    println!("=================");
                    println!("Project: {}", config.name);
                    println!("Output:  {}", config.output_dir.display());
                    println!();
                    
                    println!("📝 Content:");
                    println!("  Markdown files: {}", files.markdown_files.len());
                    println!("  Mermaid files:  {}", files.mermaid_files.len());
                    println!("  Images:         {}", files.image_files.len());
                    println!();
                    
                    if !files.markdown_files.is_empty() {
                        println!("📄 Files:");
                        for file in &files.markdown_files {
                            println!("  • {}", file.path.display());
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get project status: {}", e);
                    return Err(e);
                }
            }
        }
        
        Commands::Clean => {
            info!("Cleaning generated files");
            
            let config = config::ProjectConfig::default();
            
            if config.output_dir.exists() {
                std::fs::remove_dir_all(&config.output_dir)?;
                println!("🧹 Cleaned output directory: {}", config.output_dir.display());
            } else {
                println!("✨ Output directory already clean");
            }
        }
        
        Commands::Templates { action } => {
            let config = config::ProjectConfig::default();
            let template_manager = templates::TemplateManager::new(config.templates_dir);
            
            match action {
                Some(TemplateCommands::List) => {
                    match template_manager.list_templates() {
                        Ok(templates) => {
                            println!("📋 Available templates:");
                            if templates.is_empty() {
                                println!("  No templates installed");
                                println!("  Use 'templates download-eisvogel' to get started");
                            } else {
                                for template in templates {
                                    println!("  • {}", template);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to list templates: {}", e);
                            return Err(e);
                        }
                    }
                }
                Some(TemplateCommands::DownloadEisvogel) => {
                    match template_manager.download_eisvogel().await {
                        Ok(()) => {
                            println!("📥 Eisvogel template downloaded successfully!");
                        }
                        Err(e) => {
                            error!("Failed to download template: {}", e);
                            return Err(e);
                        }
                    }
                }
                Some(TemplateCommands::Install { path }) => {
                    match template_manager.install_template(&path) {
                        Ok(()) => {
                            println!("📦 Template installed successfully!");
                        }
                        Err(e) => {
                            error!("Failed to install template: {}", e);
                            return Err(e);
                        }
                    }
                }
                None => {
                    println!("📋 Template management");
                    println!("Available commands:");
                    println!("  list              - List installed templates");
                    println!("  download-eisvogel - Download Eisvogel LaTeX template");
                    println!("  install <path>    - Install template from file");
                }
            }
        }
        
        Commands::Diagrams => {
            info!("Generating diagrams");
            
            let config = config::ProjectConfig::default();
            let discovery = discovery::FileDiscovery::new(config.clone());
            let files = discovery.discover_all()?;
            
            if files.mermaid_files.is_empty() {
                println!("🎨 No Mermaid files found");
                println!("Create .mmd files or add Mermaid code blocks to your markdown");
                return Ok(());
            }
            
            let diagrams_dir = config.output_dir.join("diagrams");
            let processor = diagrams::DiagramProcessor::new(diagrams_dir);
            
            println!("🎨 Processing {} Mermaid diagrams...", files.mermaid_files.len());
            
            for mermaid_file in &files.mermaid_files {
                let content = fs::read_to_string(mermaid_file)?;
                let name = mermaid_file.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("diagram");
                
                match processor.process_mermaid(&content, name).await {
                    Ok(output_path) => {
                        println!("  ✅ {}: {}", name, output_path.display());
                    }
                    Err(e) => {
                        error!("Failed to process {}: {}", name, e);
                    }
                }
            }
            
            println!("🎨 Diagram processing complete!");
        }
        
        Commands::Config { action } => {
            match action {
                Some(ConfigCommands::Init) => {
                    let config_path = PathBuf::from("autodoc.yml");
                    if config_path.exists() {
                        println!("⚠️  Config file already exists: {}", config_path.display());
                        return Ok(());
                    }
                    
                    let default_config = config_file::AutoDocConfig::default();
                    default_config.save_to_file(&config_path)?;
                    println!("📝 Created config file: {}", config_path.display());
                }
                Some(ConfigCommands::Show) => {
                    if let Some(config_path) = config_file::AutoDocConfig::find_config_file() {
                        let config = config_file::AutoDocConfig::load_from_file(&config_path)?;
                        println!("📋 Configuration from: {}", config_path.display());
                        println!("{}", serde_yaml::to_string(&config).unwrap());
                    } else {
                        println!("📋 No config file found, using defaults");
                        let default_config = config_file::AutoDocConfig::default();
                        println!("{}", serde_yaml::to_string(&default_config).unwrap());
                    }
                }
                None => {
                    println!("📋 Configuration management");
                    println!("Available commands:");
                    println!("  init  - Create default config file");
                    println!("  show  - Show current configuration");
                }
            }
        }
    }
    
    Ok(())
}
