use ownsight_core::{AnalysisMode, ProgramAnalysis, VariableId};
use ownsight_driver::{create_analyzer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BackendAvailability {
    pub simple: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub code: String,
    pub filename: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub analysis: ProgramAnalysis,
    pub backend_used: String,
}

#[tauri::command]
pub fn analyze_snippet(request: AnalyzeRequest) -> Result<AnalyzeResponse, String> {
    let mode = match request.mode.as_deref() {
        Some("debug") => AnalysisMode::Debug,
        _ => AnalysisMode::Teaching,
    };
    
    let mut analyzer = create_analyzer(mode);
    let filename = request.filename.as_deref().unwrap_or("snippet.rs");
    let analysis = analyzer.analyze(&request.code, filename)
        .map_err(|e| e.to_string())?;
    
    Ok(AnalyzeResponse {
        analysis,
        backend_used: "simple".to_string(),
    })
}

#[tauri::command]
pub fn analyze_file(path: String, mode: Option<String>) -> Result<AnalyzeResponse, String> {
    let analysis_mode = match mode.as_deref() {
        Some("debug") => AnalysisMode::Debug,
        _ => AnalysisMode::Teaching,
    };
    
    let code = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let mut analyzer = create_analyzer(analysis_mode);
    let analysis = analyzer.analyze(&code, &path)
        .map_err(|e| e.to_string())?;
    
    Ok(AnalyzeResponse {
        analysis,
        backend_used: "simple".to_string(),
    })
}

#[tauri::command]
pub fn check_backend_availability() -> BackendAvailability {
    BackendAvailability {
        simple: true,
    }
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
    }
}
