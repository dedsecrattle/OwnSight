//! Tests for closure capture analysis

#[cfg(test)]
mod tests {
    use ownsight_core::*;
    
    #[test]
    fn test_capture_modes() {
        // Test that capture mode enum works correctly
        let by_value = CaptureMode::ByValue;
        let by_ref = CaptureMode::ByRef;
        let by_mut_ref = CaptureMode::ByMutRef;
        
        assert_ne!(by_value, by_ref);
        assert_ne!(by_ref, by_mut_ref);
        assert_ne!(by_value, by_mut_ref);
    }
    
    #[test]
    fn test_closure_capture_info() {
        let capture = ClosureCapture {
            var_id: VariableId(0),
            capture_mode: CaptureMode::ByRef,
            by_ref: true,
        };
        
        assert_eq!(capture.var_id, VariableId(0));
        assert_eq!(capture.capture_mode, CaptureMode::ByRef);
        assert!(capture.by_ref);
    }
}
