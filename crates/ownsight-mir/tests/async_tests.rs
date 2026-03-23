//! Tests for async/await analysis

#[cfg(test)]
mod tests {
    use ownsight_core::*;
    
    // Define SuspensionPoint locally for tests since it's in the mir crate
    #[derive(Debug, Clone)]
    pub struct SuspensionPoint {
        pub location: usize,
        pub line_number: usize,
        pub variables_live: Vec<VariableId>,
    }
    
    #[test]
    fn test_async_context_creation() {
        let context = AsyncContext {
            is_async: true,
            await_points: vec![5, 10, 15],
            send_required: true,
            sync_required: false,
        };
        
        assert!(context.is_async);
        assert_eq!(context.await_points.len(), 3);
        assert!(context.send_required);
        assert!(!context.sync_required);
    }
    
    #[test]
    fn test_suspension_point() {
        let suspension = SuspensionPoint {
            location: 42,
            line_number: 10,
            variables_live: vec![VariableId(0), VariableId(1)],
        };
        
        assert_eq!(suspension.location, 42);
        assert_eq!(suspension.line_number, 10);
        assert_eq!(suspension.variables_live.len(), 2);
    }
    
    #[test]
    fn test_async_context_no_await() {
        let context = AsyncContext {
            is_async: true,
            await_points: vec![],
            send_required: false,
            sync_required: false,
        };
        
        assert!(context.is_async);
        assert!(context.await_points.is_empty());
    }
}
