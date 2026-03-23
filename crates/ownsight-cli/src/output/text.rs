use ownsight_core::ProgramAnalysis;
use anyhow::Result;
use colored::*;

pub fn print_summary(analysis: &ProgramAnalysis) -> Result<()> {
    println!("\n{}", "=== Ownership Analysis Summary ===".bold().cyan());
    
    println!("\n{}", "Variables:".bold());
    for var in &analysis.variables {
        let mutability = if var.is_mutable { "mut" } else { "   " };
        println!("  {} {} {}: {}", 
            mutability.yellow(),
            var.name.green(),
            "type".dimmed(),
            var.ty.blue()
        );
    }
    
    println!("\n{}", "Ownership Graph:".bold());
    println!("  Nodes: {}", analysis.ownership_graph.nodes.len());
    println!("  Edges: {}", analysis.ownership_graph.edges.len());
    
    for edge in &analysis.ownership_graph.edges {
        println!("    {:?} -> {:?} ({:?})", edge.source, edge.target, edge.kind);
    }
    
    Ok(())
}
