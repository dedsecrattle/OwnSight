use ownsight_core::*;
use anyhow::Result;
use std::collections::HashMap;

pub struct SimpleAnalyzer {
    mode: AnalysisMode,
}

impl SimpleAnalyzer {
    pub fn new(mode: AnalysisMode) -> Self {
        Self { mode }
    }
    
    pub fn analyze(&mut self, source: &str, filename: &str) -> Result<ProgramAnalysis> {
        let mut analyzer = Analyzer::new(self.mode.clone());
        
        analyzer.analyze_snippet(source, filename)?;
        
        let parsed = self.parse_simple_rust(source, filename)?;
        
        analyzer.analysis.variables = parsed.variables;
        analyzer.analysis.events = parsed.events;
        analyzer.analysis.scopes = parsed.scopes;
        analyzer.analysis.functions = parsed.functions;
        
        Ok(analyzer.finalize())
    }
    
    fn parse_simple_rust(&self, source: &str, filename: &str) -> Result<ParsedProgram> {
        let mut parsed = ParsedProgram::default();
        let lines: Vec<&str> = source.lines().collect();
        
        let mut var_counter = 0;
        let _event_counter = 0;
        let mut var_map: HashMap<String, VariableId> = HashMap::new();
        let mut event_builder = EventBuilder::new();
        
        let root_scope = Scope {
            id: ScopeId(0),
            parent: None,
            start_line: 1,
            end_line: lines.len(),
            kind: ScopeKind::Function,
        };
        parsed.scopes.push(root_scope);
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx + 1;
            let trimmed = line.trim();
            
            if trimmed.starts_with("let ") {
                if let Some(var_info) = self.parse_let_statement(trimmed, line_num, filename) {
                    let var_id = VariableId(var_counter);
                    var_counter += 1;
                    
                    let variable = Variable {
                        id: var_id,
                        name: var_info.name.clone(),
                        ty: var_info.ty.clone(),
                        scope_id: ScopeId(0),
                        span: Span::single_line(filename.to_string(), line_num, 0, line.len()),
                        is_mutable: var_info.is_mutable,
                    };
                    
                    var_map.insert(var_info.name.clone(), var_id);
                    parsed.variables.push(variable);
                    
                    let create_event = event_builder.create_detailed_event(
                        EventKind::Create,
                        var_id,
                        Span::single_line(filename.to_string(), line_num, 0, line.len()),
                        line_num,
                        &var_info.name,
                        None,
                    );
                    parsed.events.push(create_event);
                    
                    if trimmed.contains("&mut ") {
                        if let Some(borrowed_name) = self.extract_borrowed_var(trimmed) {
                            if let Some(&borrowed_id) = var_map.get(&borrowed_name) {
                                let mut borrow_event = event_builder.create_detailed_event(
                                    EventKind::BorrowMut,
                                    borrowed_id,
                                    Span::single_line(filename.to_string(), line_num, 0, line.len()),
                                    line_num,
                                    &borrowed_name,
                                    None,
                                );
                                borrow_event.related_variable_id = Some(var_id);
                                parsed.events.push(borrow_event);
                            }
                        }
                    } else if trimmed.contains("&") && !trimmed.contains("&mut") {
                        if let Some(borrowed_name) = self.extract_borrowed_var(trimmed) {
                            if let Some(&borrowed_id) = var_map.get(&borrowed_name) {
                                let mut borrow_event = event_builder.create_detailed_event(
                                    EventKind::BorrowShared,
                                    borrowed_id,
                                    Span::single_line(filename.to_string(), line_num, 0, line.len()),
                                    line_num,
                                    &borrowed_name,
                                    None,
                                );
                                borrow_event.related_variable_id = Some(var_id);
                                parsed.events.push(borrow_event);
                            }
                        }
                    }
                }
            }
            
            if let Some(func_call) = self.parse_function_call(trimmed) {
                for arg in &func_call.args {
                    if let Some(&var_id) = var_map.get(arg) {
                        if !trimmed.contains(&format!("&{}", arg)) {
                            let move_event = event_builder.create_detailed_event(
                                EventKind::MoveOut,
                                var_id,
                                Span::single_line(filename.to_string(), line_num, 0, line.len()),
                                line_num,
                                arg,
                                Some(format!(
                                    "`{}` was moved into function `{}`. It cannot be used after this point.",
                                    arg, func_call.name
                                )),
                            );
                            parsed.events.push(move_event);
                        }
                    }
                }
            }
            
            if trimmed == "}" {
                for (var_name, &var_id) in &var_map {
                    let last_event = parsed.events.iter()
                        .filter(|e| e.variable_id == var_id)
                        .last();
                    
                    if let Some(last) = last_event {
                        if last.kind != EventKind::MoveOut && last.kind != EventKind::Drop {
                            let drop_event = event_builder.create_detailed_event(
                                EventKind::Drop,
                                var_id,
                                Span::single_line(filename.to_string(), line_num, 0, line.len()),
                                line_num,
                                var_name,
                                None,
                            );
                            parsed.events.push(drop_event);
                        }
                    }
                }
            }
        }
        
        Ok(parsed)
    }
    
    fn parse_let_statement(&self, line: &str, _line_num: usize, _filename: &str) -> Option<VarInfo> {
        let is_mutable = line.contains("let mut ");
        let after_let = if is_mutable {
            line.strip_prefix("let mut ")?.trim()
        } else {
            line.strip_prefix("let ")?.trim()
        };
        
        let parts: Vec<&str> = after_let.split('=').collect();
        if parts.is_empty() {
            return None;
        }
        
        let var_part = parts[0].trim();
        let name_and_type: Vec<&str> = var_part.split(':').collect();
        
        let name = name_and_type[0].trim().to_string();
        let ty = if name_and_type.len() > 1 {
            name_and_type[1].trim().to_string()
        } else {
            "inferred".to_string()
        };
        
        Some(VarInfo { name, ty, is_mutable })
    }
    
    fn extract_borrowed_var(&self, line: &str) -> Option<String> {
        if let Some(pos) = line.find("&mut ") {
            let after = &line[pos + 5..];
            let var_name = after.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()?
                .trim();
            return Some(var_name.to_string());
        } else if let Some(pos) = line.find("&") {
            let after = &line[pos + 1..];
            let var_name = after.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()?
                .trim();
            if !var_name.is_empty() && var_name != "mut" {
                return Some(var_name.to_string());
            }
        }
        None
    }
    
    fn parse_function_call(&self, line: &str) -> Option<FunctionCall> {
        if !line.contains('(') || !line.contains(')') {
            return None;
        }
        
        let parts: Vec<&str> = line.split('(').collect();
        if parts.len() < 2 {
            return None;
        }
        
        let func_name = parts[0].trim().split_whitespace().last()?.to_string();
        
        let args_part = parts[1].split(')').next()?;
        let args: Vec<String> = args_part
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.trim_start_matches('&')
                    .trim_start_matches("mut ")
                    .trim()
                    .to_string()
            })
            .collect();
        
        Some(FunctionCall {
            name: func_name,
            args,
        })
    }
}

#[derive(Default)]
struct ParsedProgram {
    variables: Vec<Variable>,
    events: Vec<Event>,
    scopes: Vec<Scope>,
    functions: Vec<FunctionInfo>,
}

struct VarInfo {
    name: String,
    ty: String,
    is_mutable: bool,
}

struct FunctionCall {
    name: String,
    args: Vec<String>,
}
