use ownsight_core::{AnalysisMode, ProgramAnalysis, VariableId};
use ownsight_driver::{AnalyzerBackend, create_analyzer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BackendAvailability {
    pub simple: bool,
    pub mir: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub code: String,
    pub filename: Option<String>,
    pub mode: Option<String>,
    pub backend: Option<String>,
}

#[tauri::command]
pub fn analyze_snippet(request: AnalyzeRequest) -> Result<ProgramAnalysis, String> {
    let mode = match request.mode.as_deref() {
        Some("debug") => AnalysisMode::Debug,
        _ => AnalysisMode::Teaching,
    };
    
    let backend = match request.backend.as_deref() {
        Some("mir") => AnalyzerBackend::Mir,
        Some("simple") => AnalyzerBackend::Simple,
        _ => AnalyzerBackend::default(),
    };
    
    let filename = request.filename.as_deref().unwrap_or("snippet.rs");
    
    let mut analyzer = create_analyzer(backend, mode);
    analyzer.analyze(&request.code, filename)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn analyze_file(path: String, mode: Option<String>, backend: Option<String>) -> Result<ProgramAnalysis, String> {
    let code = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let analysis_mode = match mode.as_deref() {
        Some("debug") => AnalysisMode::Debug,
        _ => AnalysisMode::Teaching,
    };
    
    let analyzer_backend = match backend.as_deref() {
        Some("mir") => AnalyzerBackend::Mir,
        Some("simple") => AnalyzerBackend::Simple,
        _ => AnalyzerBackend::default(),
    };
    
    let mut analyzer = create_analyzer(analyzer_backend, analysis_mode);
    analyzer.analyze(&code, &path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn query_why_cant_use(
    analysis_json: String,
    var_id: usize,
    line: usize,
) -> Result<Option<String>, String> {
    let analysis: ProgramAnalysis = serde_json::from_str(&analysis_json)
        .map_err(|e| format!("Failed to parse analysis: {}", e))?;
    
    let analyzer = ownsight_core::Analyzer {
        analysis,
    };
    
    Ok(analyzer.query_why_cant_use(VariableId(var_id), line))
}

#[tauri::command]
pub fn query_where_moved(
    analysis_json: String,
    var_id: usize,
) -> Result<Vec<usize>, String> {
    let analysis: ProgramAnalysis = serde_json::from_str(&analysis_json)
        .map_err(|e| format!("Failed to parse analysis: {}", e))?;
    
    let analyzer = ownsight_core::Analyzer {
        analysis,
    };
    
    Ok(analyzer.query_where_moved(VariableId(var_id)))
}

#[tauri::command]
pub fn query_what_borrows(
    analysis_json: String,
    var_id: usize,
    line: usize,
) -> Result<Vec<String>, String> {
    let analysis: ProgramAnalysis = serde_json::from_str(&analysis_json)
        .map_err(|e| format!("Failed to parse analysis: {}", e))?;
    
    let analyzer = ownsight_core::Analyzer {
        analysis,
    };
    
    Ok(analyzer.query_what_borrows(VariableId(var_id), line))
}

#[tauri::command]
pub fn check_backend_availability() -> BackendAvailability {
    BackendAvailability {
        simple: true,
        mir: cfg!(feature = "mir"),
    }
}
