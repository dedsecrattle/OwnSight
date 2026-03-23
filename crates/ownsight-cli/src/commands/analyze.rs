use anyhow::{Result, Context};
use ownsight_core::AnalysisMode;
use ownsight_driver::SimpleAnalyzer;
use std::fs;
use std::io::{self, Read};

pub fn run(
    file: Option<String>,
    stdin: bool,
    output: Option<String>,
    mode: Option<String>,
    _function: Option<String>,
) -> Result<()> {
    let (source, filename) = if stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        (buffer, "snippet.rs".to_string())
    } else if let Some(path) = file {
        let content = fs::read_to_string(&path)
            .context(format!("Failed to read file: {}", path))?;
        (content, path)
    } else {
        anyhow::bail!("Either --file or --stdin must be provided");
    };
    
    let analysis_mode = match mode.as_deref() {
        Some("debug") => AnalysisMode::Debug,
        _ => AnalysisMode::Teaching,
    };
    
    let mut analyzer = SimpleAnalyzer::new(analysis_mode);
    let analysis = analyzer.analyze(&source, &filename)?;
    
    let output_format = output.as_deref().unwrap_or("timeline");
    
    match output_format {
        "json" => {
            let json = serde_json::to_string_pretty(&analysis)?;
            println!("{}", json);
        }
        "timeline" => {
            crate::output::timeline::print_timeline(&analysis)?;
        }
        "text" => {
            crate::output::text::print_summary(&analysis)?;
        }
        _ => {
            anyhow::bail!("Unknown output format: {}", output_format);
        }
    }
    
    Ok(())
}
