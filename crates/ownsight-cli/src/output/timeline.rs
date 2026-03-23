use ownsight_core::{ProgramAnalysis, EventKind};
use anyhow::Result;
use colored::*;

pub fn print_timeline(analysis: &ProgramAnalysis) -> Result<()> {
    println!("\n{}", "=== Ownership Timeline ===".bold().cyan());
    println!("{}: {}\n", "Mode".bold(), format!("{:?}", analysis.metadata.mode).green());
    
    if analysis.files.is_empty() {
        println!("{}", "No source files analyzed".yellow());
        return Ok(());
    }
    
    let source_file = &analysis.files[0];
    println!("{}: {}\n", "File".bold(), source_file.path.blue());
    
    println!("{}", "Source Code:".bold());
    for (idx, line) in source_file.lines.iter().enumerate() {
        let line_num = idx + 1;
        println!("{:4} | {}", line_num.to_string().dimmed(), line);
    }
    println!();
    
    println!("{}", "Events:".bold());
    let mut sorted_events = analysis.events.clone();
    sorted_events.sort_by_key(|e| e.line_number);
    
    for event in &sorted_events {
        let var = analysis.get_variable(event.variable_id);
        let var_name = var.map(|v| v.name.as_str()).unwrap_or("?");
        
        let event_icon = match event.kind {
            EventKind::Create | EventKind::StorageLive => "✨",
            EventKind::MoveOut => "📦",
            EventKind::MoveIn => "📥",
            EventKind::BorrowShared => "👁️ ",
            EventKind::BorrowMut => "✏️ ",
            EventKind::Use => "🔍",
            EventKind::Drop | EventKind::StorageDead => "🗑️ ",
            EventKind::Conflict => "⚠️ ",
            _ => "•",
        };
        
        let event_color = match event.kind {
            EventKind::Create | EventKind::StorageLive => "green",
            EventKind::MoveOut => "red",
            EventKind::MoveIn => "blue",
            EventKind::BorrowShared => "cyan",
            EventKind::BorrowMut => "yellow",
            EventKind::Drop | EventKind::StorageDead => "magenta",
            EventKind::Conflict => "red",
            _ => "white",
        };
        
        let line_str = format!("Line {}", event.line_number);
        let kind_str = format!("{:?}", event.kind);
        
        print!("{} {} ", event_icon, line_str.dimmed());
        
        match event_color {
            "green" => print!("{}: ", kind_str.green()),
            "red" => print!("{}: ", kind_str.red()),
            "blue" => print!("{}: ", kind_str.blue()),
            "cyan" => print!("{}: ", kind_str.cyan()),
            "yellow" => print!("{}: ", kind_str.yellow()),
            "magenta" => print!("{}: ", kind_str.magenta()),
            _ => print!("{}: ", kind_str),
        }
        
        println!("`{}` - {}", var_name.bold(), event.explanation);
        
        if let Some(related_id) = event.related_variable_id {
            if let Some(related_var) = analysis.get_variable(related_id) {
                println!("       {} Related to: `{}`", "↳".dimmed(), related_var.name.bold());
            }
        }
    }
    
    println!("\n{}", "=== Summary ===".bold().cyan());
    println!("Variables: {}", analysis.variables.len().to_string().green());
    println!("Events: {}", analysis.events.len().to_string().green());
    println!("Scopes: {}", analysis.scopes.len().to_string().green());
    
    if !analysis.diagnostics.is_empty() {
        println!("\n{}", "=== Diagnostics ===".bold().yellow());
        for diag in &analysis.diagnostics {
            println!("{}: {}", format!("{:?}", diag.level).red(), diag.message);
        }
    }
    
    Ok(())
}
