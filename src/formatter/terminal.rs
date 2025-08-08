use crate::{TerraformPlan, ActionType};
use colored::*;

pub fn format_terminal_output(plan: &TerraformPlan) -> String {
    let mut output = String::new();
    
    // Header
    output.push_str(&"═".repeat(70).bright_blue().to_string());
    output.push('\n');
    output.push_str(&format!("                    {} TERRAFORM {} DIFF                    ", 
        "▶".bright_blue(), 
        match plan.mode {
            crate::PlanMode::Plan => "PLAN".bright_green(),
            crate::PlanMode::Apply => "APPLY".bright_yellow(),
        }
    ));
    output.push('\n');
    output.push_str(&"═".repeat(70).bright_blue().to_string());
    output.push('\n');
    
    // Summary box
    if plan.summary.add > 0 || plan.summary.change > 0 || plan.summary.destroy > 0 || plan.summary.read > 0 {
        output.push_str("\n                    ┌────────────────────────────────┐\n");
        output.push_str(&format!("                    │ {} │\n", 
            format_summary_line(&plan.summary)
        ));
        output.push_str("                    └────────────────────────────────┘\n");
    }
    
    output.push('\n');
    output.push_str(&"═".repeat(70).bright_blue().to_string());
    output.push('\n');
    
    // Resources
    for resource in &plan.resources {
        output.push_str(&format_resource(resource));
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
    
    let (symbol, color, action_text) = match resource.action {
        ActionType::Create => ("✚", "bright_green", "CREATE"),
        ActionType::Update => ("↻", "bright_yellow", "UPDATE"), 
        ActionType::Destroy => ("✖", "bright_red", "DESTROY"),
        ActionType::Read => ("⇐", "bright_cyan", "READ"),
        ActionType::NoOp => ("○", "bright_white", "NO-OP"),
    };
    
    // Resource header
    let header = format!("{} {} {}", symbol, action_text, resource.id);
    output.push_str(&match color {
        "bright_green" => header.bright_green().to_string(),
        "bright_yellow" => header.bright_yellow().to_string(),
        "bright_red" => header.bright_red().to_string(),
        "bright_cyan" => header.bright_cyan().to_string(),
        _ => header.bright_white().to_string(),
    });
    output.push('\n');
    
    // Separator line
    output.push_str(&"─".repeat(70).dimmed().to_string());
    output.push('\n');
    
    // TODO: Add attribute formatting
    for (key, value) in &resource.attributes {
        output.push_str(&format!("  {} = {}\n", 
            key.bright_white(),
            format!("{}", value).dimmed()
        ));
    }
    
    output
}