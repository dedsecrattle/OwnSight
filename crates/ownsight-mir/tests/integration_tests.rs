//! Integration tests for MIR analyzer

use ownsight_mir::MirAnalyzer;
use ownsight_core::AnalysisMode;

#[test]
fn test_mir_analyzer_creation() {
    let analyzer = MirAnalyzer::new(AnalysisMode::Teaching);
    // Just verify it can be created
    drop(analyzer);
}

#[test]
fn test_mir_analyzer_debug_mode() {
    let analyzer = MirAnalyzer::new(AnalysisMode::Debug);
    drop(analyzer);
}

#[test]
#[cfg(not(feature = "rustc"))]
fn test_mir_analyzer_without_rustc_feature() {
    let mut analyzer = MirAnalyzer::new(AnalysisMode::Teaching);
    
    // Should return error when rustc feature is not enabled
    let result = analyzer.analyze("fn main() {}", "test.rs");
    assert!(result.is_err());
    
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("rustc"));
}
