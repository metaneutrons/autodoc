use crate::errors::{AutoDocError, Result};
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, debug};
use std::process::Command;

pub struct DiagramProcessor {
    output_dir: PathBuf,
}

impl DiagramProcessor {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }
    
    pub async fn process_mermaid(&self, mermaid_code: &str, output_name: &str) -> Result<PathBuf> {
        info!("Processing Mermaid diagram: {}", output_name);
        
        self.ensure_output_dir()?;
        
        // Try native rendering first, fallback to file save
        match self.render_with_mermaid_cli(mermaid_code, output_name).await {
            Ok(path) => Ok(path),
            Err(_) => {
                debug!("Mermaid CLI not available, saving source code");
                let mermaid_path = self.output_dir.join(format!("{}.mmd", output_name));
                fs::write(&mermaid_path, mermaid_code)?;
                info!("💾 Mermaid source saved: {}", mermaid_path.display());
                Ok(mermaid_path)
            }
        }
    }
    
    async fn render_with_mermaid_cli(&self, mermaid_code: &str, output_name: &str) -> Result<PathBuf> {
        // Save mermaid source
        let input_path = self.output_dir.join(format!("{}.mmd", output_name));
        fs::write(&input_path, mermaid_code)?;
        
        // Try to render with mermaid CLI
        let svg_path = self.output_dir.join(format!("{}.svg", output_name));
        
        let output = Command::new("mmdc")
            .args(&[
                "-i", input_path.to_str().unwrap(),
                "-o", svg_path.to_str().unwrap(),
                "-t", "neutral",
                "-b", "white"
            ])
            .output()
            .map_err(|e| AutoDocError::Build { 
                message: format!("Mermaid CLI not found: {}", e) 
            })?;
        
        if !output.status.success() {
            return Err(AutoDocError::Build { 
                message: "Mermaid rendering failed".to_string() 
            });
        }
        
        info!("🎨 Mermaid diagram rendered: {}", svg_path.display());
        
        // Convert to PDF for LaTeX compatibility
        let pdf_path = self.output_dir.join(format!("{}.pdf", output_name));
        self.svg_to_pdf_with_inkscape(&svg_path, &pdf_path)?;
        
        Ok(pdf_path)
    }
    
    fn svg_to_pdf_with_inkscape(&self, svg_path: &Path, pdf_path: &Path) -> Result<()> {
        let output = Command::new("inkscape")
            .args(&[
                svg_path.to_str().unwrap(),
                "--export-type=pdf",
                &format!("--export-filename={}", pdf_path.to_str().unwrap())
            ])
            .output();
        
        match output {
            Ok(result) if result.status.success() => {
                debug!("📄 SVG converted to PDF: {}", pdf_path.display());
                Ok(())
            }
            _ => {
                debug!("Inkscape not available, keeping SVG");
                Ok(())
            }
        }
    }
    
    pub fn ensure_output_dir(&self) -> Result<()> {
        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir)?;
            info!("Created diagrams directory: {}", self.output_dir.display());
        }
        Ok(())
    }
}
