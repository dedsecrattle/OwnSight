//! Tests for partial move analysis

#[cfg(test)]
mod tests {
    use ownsight_core::*;
    
    #[test]
    fn test_partial_move_detection() {
        // This test would require the full MIR backend to be functional
        // For now, we verify the data structures are correct
        
        let partial_move = PartialMoveInfo {
            base_var: VariableId(0),
            field_path: vec!["x".to_string()],
            moved_fields: vec!["x".to_string()],
        };
        
        assert_eq!(partial_move.base_var, VariableId(0));
        assert_eq!(partial_move.moved_fields.len(), 1);
        assert_eq!(partial_move.field_path.len(), 1);
    }
    
    #[test]
    fn test_field_path_construction() {
        let path = vec!["field_0".to_string(), "field_1".to_string()];
        let joined = path.join(".");
        assert_eq!(joined, "field_0.field_1");
    }
}
