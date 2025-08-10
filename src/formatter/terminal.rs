use crate::{TerraformPlan, ActionType, Change};
use colored::*;
use std::collections::HashMap;
use serde_json::Value;

pub fn format_terminal_output(plan: &TerraformPlan) -> String {
    let mut output = String::new();
    
    // Header with improved styling
    output.push_str(&"╔".bright_blue().to_string());
    output.push_str(&"═".repeat(78).bright_blue().to_string());
    output.push_str(&"╗".bright_blue().to_string());
    output.push('\n');
    
    let mode_text = match plan.mode {
        crate::PlanMode::Plan => "TERRAFORM PLAN ANALYSIS".bright_cyan(),
        crate::PlanMode::Apply => "TERRAFORM APPLY ANALYSIS".bright_yellow(),
    };
    output.push_str(&format!("║{:^78}║\n", mode_text));
    
    output.push_str(&"╚".bright_blue().to_string());
    output.push_str(&"═".repeat(78).bright_blue().to_string());
    output.push_str(&"╝".bright_blue().to_string());
    output.push('\n');
    output.push('\n');
    
    // Enhanced Summary Section
    if plan.summary.add > 0 || plan.summary.change > 0 || plan.summary.destroy > 0 || plan.summary.read > 0 {
        output.push_str(&"📊 ".to_string());
        output.push_str(&"PLAN SUMMARY".bright_white().bold().to_string());
        output.push('\n');
        output.push_str(&"─".repeat(80).dimmed().to_string());
        output.push('\n');
        output.push('\n');
        
        // Summary stats in a grid layout
        if plan.summary.add > 0 {
            output.push_str(&format!("  {} {} resources to add\n", 
                "➕".bright_green(),
                plan.summary.add.to_string().bright_green().bold()
            ));
        }
        if plan.summary.change > 0 {
            output.push_str(&format!("  {} {} resources to change\n", 
                "🔄".bright_yellow(),
                plan.summary.change.to_string().bright_yellow().bold()
            ));
        }
        if plan.summary.destroy > 0 {
            output.push_str(&format!("  {} {} resources to destroy\n", 
                "🗑️".bright_red(),
                plan.summary.destroy.to_string().bright_red().bold()
            ));
        }
        if plan.summary.read > 0 {
            output.push_str(&format!("  {} {} data sources to read\n", 
                "📖".bright_cyan(),
                plan.summary.read.to_string().bright_cyan().bold()
            ));
        }
        output.push('\n');
        output.push_str(&"═".repeat(80).bright_blue().to_string());
        output.push('\n');
        output.push('\n');
    }
    
    // Resources with enhanced formatting
    if !plan.resources.is_empty() {
        output.push_str(&"🔧 ".to_string());
        output.push_str(&"RESOURCES".bright_white().bold().to_string());
        output.push('\n');
        output.push_str(&"─".repeat(80).dimmed().to_string());
        output.push('\n');
        output.push('\n');
        
        for resource in &plan.resources {
            output.push_str(&format_resource(resource));
            output.push('\n');
        }
    }
    
    // Data sources section
    if !plan.data_sources.is_empty() {
        output.push_str(&"📊 ".to_string());
        output.push_str(&"DATA SOURCES".bright_white().bold().to_string());
        output.push('\n');
        output.push_str(&"─".repeat(80).dimmed().to_string());
        output.push('\n');
        output.push('\n');
        
        for data_source in &plan.data_sources {
            output.push_str(&format_data_source(data_source));
            output.push('\n');
        }
    }
    
    // Warnings section
    if !plan.warnings.is_empty() {
        output.push_str(&"⚠️  ".to_string());
        output.push_str(&"WARNINGS".bright_yellow().bold().to_string());
        output.push('\n');
        output.push_str(&"─".repeat(80).dimmed().to_string());
        output.push('\n');
        output.push('\n');
        
        for warning in &plan.warnings {
            output.push_str(&format!("  {} {}\n", 
                "•".bright_yellow(),
                warning.message.bright_yellow()
            ));
        }
        output.push('\n');
    }
    
    output
}

pub fn format_summary_line(summary: &crate::Summary) -> String {
    let mut parts = Vec::new();
    
    if summary.add > 0 {
        parts.push(format!("✚ {} to add", summary.add).bright_green().to_string());
    }
    if summary.change > 0 {
        parts.push(format!("↻ {} to change", summary.change).bright_yellow().to_string());
    }
    if summary.destroy > 0 {
        parts.push(format!("✖ {} to destroy", summary.destroy).bright_red().to_string());
    }
    if summary.read > 0 {
        parts.push(format!("⇐ {} to read", summary.read).bright_cyan().to_string());
    }
    
    parts.join("  │  ")
}

pub fn format_resource(resource: &crate::Resource) -> String {
    let mut output = String::new();
    
    // Resource header with action badge
    let action_badge = match resource.action {
        ActionType::Create => format!(" {} ", "CREATE").on_bright_green().black().bold(),
        ActionType::Update => format!(" {} ", "UPDATE").on_bright_yellow().black().bold(),
        ActionType::Destroy => format!(" {} ", "DESTROY").on_bright_red().white().bold(),
        ActionType::Read => format!(" {} ", "READ").on_bright_cyan().black().bold(),
        ActionType::NoOp => format!(" {} ", "NO-OP").on_bright_black().white().bold(),
    };
    
    output.push_str(&format!("{} {}\n", action_badge, resource.id.bright_white().bold()));
    
    // Show resource type and provider if available
    if !resource.type_name.is_empty() {
        output.push_str(&format!("  {} {}\n", 
            "Type:".dimmed(),
            resource.type_name.bright_blue()
        ));
    }
    
    // If there are changes, show them in a diff-like format
    if !resource.changes.is_empty() {
        output.push('\n');
        output.push_str(&format_changes(&resource.changes, &resource.action));
    }
    
    // Show attributes if no changes are present (for backwards compatibility)
    if resource.changes.is_empty() && !resource.attributes.is_empty() {
        output.push('\n');
        for (key, value) in &resource.attributes {
            // Check if this attribute has arrow notation for side-by-side display
            if let Value::String(val_str) = value {
                if val_str.contains(" → ") {
                    let parts: Vec<&str> = val_str.splitn(2, " → ").collect();
                    if parts.len() == 2 {
                        let before = parts[0].trim();
                        let after = parts[1].trim();
                        
                        // For DESTROY resources with "→ null", just show the removal
                        if resource.action == ActionType::Destroy && after == "null" {
                            output.push_str(&format!("  {} {} = {}\n",
                                "-".bright_red().bold(),
                                key.bright_cyan(),
                                before.bright_red()
                            ));
                            continue;
                        }
                        
                        output.push_str(&format!("  {} {}\n", 
                            "~".bright_yellow().bold(),
                            key.bright_cyan()
                        ));
                        
                        let max_width = 40;
                        let before_truncated = if before.len() > max_width {
                            format!("{}...", &before[..max_width-3])
                        } else {
                            before.to_string()
                        };
                        let after_truncated = if after.len() > max_width {
                            format!("{}...", &after[..max_width-3])
                        } else {
                            after.to_string()
                        };
                        
                        output.push_str(&format!("    {} {:<40} │ {} {}\n",
                            "─".bright_red(),
                            before_truncated.bright_red(),
                            "+".bright_green(),
                            after_truncated.bright_green()
                        ));
                        continue;
                    }
                }
            }
            
            let formatted_value = format_json_value(value, 1);
            output.push_str(&format!("  {} = {}\n", 
                key.bright_cyan(),
                formatted_value
            ));
        }
    }
    
    output.push_str(&"─".repeat(80).dimmed().to_string());
    output.push('\n');
    
    output
}

fn format_changes(changes: &[Change], _action: &ActionType) -> String {
    let mut output = String::new();
    
    // Group changes by their path for better organization
    let mut grouped_changes: HashMap<String, Vec<&Change>> = HashMap::new();
    for change in changes {
        let path_key = if change.path.is_empty() {
            "root".to_string()
        } else {
            change.path[0].clone()
        };
        grouped_changes.entry(path_key).or_insert_with(Vec::new).push(change);
    }
    
    for change in changes {
        let path_str = if change.path.is_empty() {
            "(root)".to_string()
        } else {
            change.path.join(".")
        };
        
        // Handle different change scenarios
        match (&change.before, &change.after) {
            (None, Some(after)) => {
                // Creation
                output.push_str(&format!("  {} {} = {}\n",
                    "+".bright_green().bold(),
                    path_str.bright_cyan(),
                    format_json_value(after, 2).bright_green()
                ));
            },
            (Some(before), None) => {
                // Deletion
                output.push_str(&format!("  {} {} = {}\n",
                    "-".bright_red().bold(),
                    path_str.bright_cyan(),
                    format_json_value(before, 2).bright_red()
                ));
            },
            (Some(before), Some(after)) if before != after => {
                // Check if this is a DESTROY action with null after value
                if let ActionType::Destroy = _action {
                    if after == &Value::Null {
                        // For DESTROY with null, just show the value being removed
                        output.push_str(&format!("  {} {} = {}\n",
                            "-".bright_red().bold(),
                            path_str.bright_cyan(),
                            format_json_value(before, 2).bright_red()
                        ));
                        continue;
                    }
                }
                
                // Modification - show side by side with aligned columns
                let before_formatted = format_json_value(before, 0);
                let after_formatted = format_json_value(after, 0);
                
                output.push_str(&format!("  {} {}\n", 
                    "~".bright_yellow().bold(),
                    path_str.bright_cyan()
                ));
                
                // Calculate column widths for alignment
                let max_width = 40; // Maximum width for each column
                let before_truncated = if before_formatted.len() > max_width {
                    format!("{}...", &before_formatted[..max_width-3])
                } else {
                    before_formatted.clone()
                };
                let after_truncated = if after_formatted.len() > max_width {
                    format!("{}...", &after_formatted[..max_width-3])
                } else {
                    after_formatted.clone()
                };
                
                // Show side-by-side with clear visual separation
                output.push_str(&format!("    {} {:<40} │ {} {}\n",
                    "─".bright_red(),
                    before_truncated.bright_red(),
                    "+".bright_green(),
                    after_truncated.bright_green()
                ));
            },
            _ => {
                // No change or computed values
                if change.computed {
                    output.push_str(&format!("  {} {} = {}\n",
                        "?".bright_blue().bold(),
                        path_str.bright_cyan(),
                        "(known after apply)".italic().dimmed()
                    ));
                }
            }
        }
        
        if change.sensitive {
            output.push_str(&format!("    {} {}\n",
                "🔒".to_string(),
                "(sensitive value)".italic().dimmed()
            ));
        }
    }
    
    output
}

fn format_json_value(value: &Value, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level);
    
    match value {
        Value::Null => "null".dimmed().to_string(),
        Value::Bool(b) => b.to_string().bright_magenta().to_string(),
        Value::Number(n) => n.to_string().bright_blue().to_string(),
        Value::String(s) => format!("\"{}\"", s).bright_green().to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else if arr.len() == 1 {
                format!("[{}]", format_json_value(&arr[0], 0))
            } else {
                let mut result = "[\n".to_string();
                for (i, item) in arr.iter().enumerate() {
                    result.push_str(&format!("{}  {}",
                        indent,
                        format_json_value(item, indent_level + 1)
                    ));
                    if i < arr.len() - 1 {
                        result.push(',');
                    }
                    result.push('\n');
                }
                result.push_str(&format!("{}]", indent));
                result
            }
        },
        Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let mut result = "{\n".to_string();
                let entries: Vec<_> = obj.iter().collect();
                for (i, (key, val)) in entries.iter().enumerate() {
                    result.push_str(&format!("{}  {} = {}",
                        indent,
                        key.bright_cyan(),
                        format_json_value(val, indent_level + 1)
                    ));
                    if i < entries.len() - 1 {
                        result.push(',');
                    }
                    result.push('\n');
                }
                result.push_str(&format!("{}}}", indent));
                result
            }
        }
    }
}

fn format_data_source(data_source: &crate::DataSource) -> String {
    let mut output = String::new();
    
    let action_badge = format!(" {} ", "READ").on_bright_cyan().black().bold();
    output.push_str(&format!("{} {}\n", action_badge, data_source.id.bright_white().bold()));
    
    if !data_source.type_name.is_empty() {
        output.push_str(&format!("  {} {}\n", 
            "Type:".dimmed(),
            data_source.type_name.bright_blue()
        ));
    }
    
    output.push('\n');
    for (key, value) in &data_source.attributes {
        let formatted_value = format_json_value(value, 1);
        output.push_str(&format!("  {} = {}\n", 
            key.bright_cyan(),
            formatted_value
        ));
    }
    
    output.push_str(&"─".repeat(80).dimmed().to_string());
    output.push('\n');
    
    output
}