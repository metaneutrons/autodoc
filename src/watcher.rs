use crate::errors::{AutoDocError, Result};
use crate::config::ProjectConfig;
use crate::discovery::FileDiscovery;
use crate::builders::{PdfBuilder, DocxBuilder, HtmlBuilder};
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Event, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;
use tracing::{info, error};

pub struct FileWatcher {
    config: ProjectConfig,
}

impl FileWatcher {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }
    
    pub async fn watch_and_build(&self, format: &str) -> Result<()> {
        info!("Starting file watcher for {} format", format);
        
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res: notify::Result<Event>| {
                match res {
                    Ok(event) => {
                        if let Err(e) = tx.send(event) {
                            error!("Failed to send event: {}", e);
                        }
                    }
                    Err(e) => error!("Watch error: {}", e),
                }
            },
            notify::Config::default().with_poll_interval(Duration::from_secs(1))
        ).map_err(|e| AutoDocError::Build { 
            message: format!("Failed to create file watcher: {}", e) 
        })?;
        
        // Watch current directory
        watcher.watch(Path::new("."), RecursiveMode::Recursive)
            .map_err(|e| AutoDocError::Build { 
                message: format!("Failed to start watching: {}", e) 
            })?;
        
        info!("👀 Watching for changes... Press Ctrl+C to stop");
        
        // Initial build
        self.build_format(format).await?;
        
        loop {
            match rx.recv() {
                Ok(event) => {
                    match event.kind {
                        EventKind::Modify(_) | EventKind::Create(_) => {
                            for path in event.paths {
                                if self.should_rebuild(&path) {
                                    info!("File changed: {}", path.display());
                                    if let Err(e) = self.build_format(format).await {
                                        error!("Build failed: {}", e);
                                    } else {
                                        info!("✅ Rebuild complete");
                                    }
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    error!("Watch error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn should_rebuild(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("md") | Some("yaml") | Some("yml") => true,
                _ => false,
            }
        } else {
            false
        }
    }
    
    async fn build_format(&self, format: &str) -> Result<()> {
        let discovery = FileDiscovery::new(self.config.clone());
        let files = discovery.discover_all()?;
        
        if files.markdown_files.is_empty() {
            return Err(AutoDocError::Build { 
                message: "No markdown files found".to_string() 
            });
        }
        
        match format {
            "pdf" => {
                let builder = PdfBuilder::new(self.config.clone());
                builder.ensure_output_dir()?;
                let output_path = self.config.output_dir.join(format!("{}.pdf", self.config.name));
                builder.build(&files.markdown_files, &output_path).await?;
            }
            "docx" => {
                let builder = DocxBuilder::new(self.config.clone());
                builder.ensure_output_dir()?;
                let output_path = self.config.output_dir.join(format!("{}.docx", self.config.name));
                builder.build(&files.markdown_files, &output_path).await?;
            }
            "html" => {
                let builder = HtmlBuilder::new(self.config.clone());
                builder.ensure_output_dir()?;
                let output_path = self.config.output_dir.join(format!("{}.html", self.config.name));
                builder.build(&files.markdown_files, &output_path).await?;
            }
            _ => {
                return Err(AutoDocError::Build { 
                    message: format!("Unsupported format for watch: {}", format) 
                });
            }
        }
        
        Ok(())
    }
}
