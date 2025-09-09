use crate::config::ProjectConfig;
use crate::errors::{AutoDocError, Result};
use mermaid_rs::Mermaid;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

pub struct DiagramProcessor {
    config: ProjectConfig,
    mermaid: Option<Mermaid>,
}

impl DiagramProcessor {
    pub fn new(config: ProjectConfig) -> Self {
        let mermaid = match Mermaid::new() {
            Ok(m) => {
                info!("✅ Native Mermaid renderer initialized");
                Some(m)
            }
            Err(e) => {
                warn!("Failed to initialize native Mermaid renderer: {}", e);
                warn!("Diagram processing will be skipped");
                None
            }
        };

        Self { config, mermaid }
    }

    pub async fn process_all(&self, mermaid_files: &[PathBuf]) -> Result<()> {
        if self.mermaid.is_none() {
            return Err(AutoDocError::Build {
                message: "Native Mermaid renderer not available".to_string(),
            });
        }

        let mermaid = self.mermaid.as_ref().unwrap();
        let output_dir = self.config.output_dir.join("diagrams");
        fs::create_dir_all(&output_dir)?;

        for file_path in mermaid_files {
            self.process_file(mermaid, file_path, &output_dir).await?;
        }

        Ok(())
    }

    async fn process_file(
        &self,
        mermaid: &Mermaid,
        file_path: &Path,
        output_dir: &Path,
    ) -> Result<()> {
        info!("Processing diagram: {}", file_path.display());

        let content = fs::read_to_string(file_path)?;
        let file_stem = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| AutoDocError::Build {
                message: format!("Invalid filename: {}", file_path.display()),
            })?;

        // Render to SVG using native mermaid-rs
        let svg_content = mermaid.render(&content).map_err(|e| AutoDocError::Build {
            message: format!("Failed to render Mermaid diagram: {}", e),
        })?;

        // Save SVG
        let svg_path = output_dir.join(format!("{}.svg", file_stem));
        fs::write(&svg_path, &svg_content)?;
        debug!("Generated SVG: {}", svg_path.display());

        // Convert SVG to PNG using resvg
        if let Err(e) = self
            .convert_svg_to_png(&svg_content, output_dir, file_stem)
            .await
        {
            warn!("Failed to convert SVG to PNG: {}", e);
        } else {
            debug!("Generated PNG: {}/{}.png", output_dir.display(), file_stem);
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn process_inline_mermaid(&self, content: &str) -> Result<String> {
        if self.mermaid.is_none() {
            return Ok(content.to_string());
        }

        let mermaid = self.mermaid.as_ref().unwrap();
        let mut processed_content = content.to_string();

        // Find and replace ```mermaid blocks
        let mermaid_regex =
            regex::Regex::new(r"```mermaid\n(.*?)\n```").map_err(|e| AutoDocError::Build {
                message: format!("Regex error: {}", e),
            })?;

        for (i, captures) in mermaid_regex.captures_iter(content).enumerate() {
            if let Some(diagram_code) = captures.get(1) {
                match mermaid.render(diagram_code.as_str()) {
                    Ok(svg) => {
                        // Create inline SVG or save to file and reference
                        let diagram_filename = format!("inline-diagram-{}.svg", i);
                        let diagram_path = self
                            .config
                            .output_dir
                            .join("diagrams")
                            .join(&diagram_filename);

                        if fs::create_dir_all(diagram_path.parent().unwrap()).is_ok()
                            && fs::write(&diagram_path, &svg).is_ok()
                        {
                            let replacement = format!("![Diagram]({})", diagram_path.display());
                            processed_content =
                                processed_content.replace(&captures[0], &replacement);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to render inline Mermaid diagram: {}", e);
                    }
                }
            }
        }

        Ok(processed_content)
    }

    async fn convert_svg_to_png(
        &self,
        svg_content: &str,
        output_dir: &Path,
        file_stem: &str,
    ) -> Result<()> {
        use resvg::usvg::{self, TreeParsing};

        // Parse SVG
        let opt = usvg::Options::default();

        let tree = usvg::Tree::from_str(svg_content, &opt).map_err(|e| AutoDocError::Build {
            message: format!("Failed to parse SVG: {}", e),
        })?;

        let size = tree.size;
        let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32)
            .ok_or_else(|| AutoDocError::Build {
            message: "Failed to create pixmap".to_string(),
        })?;

        // Render SVG to pixmap
        resvg::Tree::from_usvg(&tree)
            .render(resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

        // Save PNG
        let png_path = output_dir.join(format!("{}.png", file_stem));
        pixmap
            .save_png(&png_path)
            .map_err(|e| AutoDocError::Build {
                message: format!("Failed to save PNG: {}", e),
            })?;

        Ok(())
    }
}
