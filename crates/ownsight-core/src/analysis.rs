use crate::model::*;
use crate::graph::*;
use anyhow::Result;

pub struct Analyzer {
    pub analysis: ProgramAnalysis,
}

impl Analyzer {
    pub fn new(mode: AnalysisMode) -> Self {
        Self {
            analysis: ProgramAnalysis::new(mode),
        }
    }
    
    pub fn analyze_snippet(&mut self, code: &str, filename: &str) -> Result<()> {
        let source_file = SourceFile::new(filename.to_string(), code.to_string());
        self.analysis.files.push(source_file);
        
        Ok(())
    }
    
    pub fn build_ownership_graph(&mut self) {
        for event in &self.analysis.events {
            let var_node = GraphNode::Variable(event.variable_id);
            self.analysis.ownership_graph.add_node(var_node.clone());
            
            match event.kind {
                EventKind::MoveOut => {
                    if let Some(related_id) = event.related_variable_id {
                        let target_node = GraphNode::Variable(related_id);
                        self.analysis.ownership_graph.add_node(target_node.clone());
                        self.analysis.ownership_graph.add_edge(
                            GraphEdge::new(var_node, target_node, EdgeKind::MovesTo)
                        );
                    }
                }
                EventKind::BorrowShared => {
                    if let Some(related_id) = event.related_variable_id {
                        let ref_node = GraphNode::Reference(related_id);
                        self.analysis.ownership_graph.add_node(ref_node.clone());
                        self.analysis.ownership_graph.add_edge(
                            GraphEdge::new(ref_node, var_node, EdgeKind::Borrows)
                        );
                    }
                }
                EventKind::BorrowMut => {
                    if let Some(related_id) = event.related_variable_id {
                        let ref_node = GraphNode::Reference(related_id);
                        self.analysis.ownership_graph.add_node(ref_node.clone());
                        self.analysis.ownership_graph.add_edge(
                            GraphEdge::new(ref_node, var_node, EdgeKind::MutablyBorrows)
                        );
                    }
                }
                _ => {}
            }
        }
    }
    
    pub fn query_why_cant_use(&self, var_id: VariableId, line: usize) -> Option<String> {
        let state = self.analysis.get_ownership_state_at_line(line);
        
        if state.is_moved(var_id) {
            let move_events: Vec<_> = self.analysis.events.iter()
                .filter(|e| e.variable_id == var_id && e.kind == EventKind::MoveOut && e.line_number < line)
                .collect();
            
            if let Some(move_event) = move_events.last() {
                let var = self.analysis.get_variable(var_id)?;
                return Some(format!(
                    "Cannot use `{}` because it was moved at line {}. {}",
                    var.name, move_event.line_number, move_event.explanation
                ));
            }
        }
        
        None
    }
    
    pub fn query_where_moved(&self, var_id: VariableId) -> Vec<usize> {
        self.analysis.events.iter()
            .filter(|e| e.variable_id == var_id && e.kind == EventKind::MoveOut)
            .map(|e| e.line_number)
            .collect()
    }
    
    pub fn query_what_borrows(&self, var_id: VariableId, line: usize) -> Vec<String> {
        let state = self.analysis.get_ownership_state_at_line(line);
        let mut results = Vec::new();
        
        for borrow in &state.active_borrows {
            if borrow.borrowed_var == var_id {
                if let Some(borrow_var_id) = borrow.borrow_var {
                    if let Some(var) = self.analysis.get_variable(borrow_var_id) {
                        let borrow_type = if borrow.is_mutable { "mutably" } else { "immutably" };
                        results.push(format!("`{}` is borrowed {} by `{}`", 
                            self.analysis.get_variable(var_id).map(|v| v.name.as_str()).unwrap_or("?"),
                            borrow_type,
                            var.name
                        ));
                    }
                }
            }
        }
        
        results
    }
    
    pub fn finalize(mut self) -> ProgramAnalysis {
        self.build_ownership_graph();
        self.analysis
    }
}
