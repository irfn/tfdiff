use crate::{TerraformPlan, ActionType};

pub fn format_html_output(plan: &TerraformPlan) -> String {
    let mut html = String::new();
    
    // HTML header
    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Terraform Diff Report</title>
    <style>
        body { font-family: 'Courier New', monospace; margin: 20px; }
        .header { text-align: center; color: #0066cc; font-weight: bold; }
        .summary { background: #f5f5f5; padding: 15px; border-radius: 5px; margin: 20px 0; }
        .resource { border: 1px solid #ddd; margin: 10px 0; border-radius: 5px; }
        .resource-header { padding: 10px; font-weight: bold; }
        .create { background-color: #d4edda; border-left: 5px solid #28a745; }
        .update { background-color: #fff3cd; border-left: 5px solid #ffc107; }
        .destroy { background-color: #f8d7da; border-left: 5px solid #dc3545; }
        .read { background-color: #d1ecf1; border-left: 5px solid #17a2b8; }
        .attributes { padding: 15px; background: #fafafa; }
    </style>
</head>
<body>"#);
    
    // Title
    html.push_str(&format!(r#"
    <div class="header">
        <h1>Terraform {} Report</h1>
    </div>"#, match plan.mode {
        crate::PlanMode::Plan => "Plan",
        crate::PlanMode::Apply => "Apply",
    }));
    
    // Summary
    html.push_str(r#"<div class="summary">"#);
    html.push_str("<h2>Summary</h2>");
    html.push_str(&format!("<p>✚ {} to add | ↻ {} to change | ✖ {} to destroy | ⇐ {} to read</p>", 
        plan.summary.add, plan.summary.change, plan.summary.destroy, plan.summary.read));
    html.push_str(r#"</div>"#);
    
    // Resources
    for resource in &plan.resources {
        let css_class = match resource.action {
            ActionType::Create => "create",
            ActionType::Update => "update",
            ActionType::Destroy => "destroy",
            ActionType::Read => "read",
            ActionType::NoOp => "noop",
        };
        
        let symbol = match resource.action {
            ActionType::Create => "✚",
            ActionType::Update => "↻",
            ActionType::Destroy => "✖",
            ActionType::Read => "⇐",
            ActionType::NoOp => "○",
        };
        
        html.push_str(&format!(r#"<div class="resource {}">
            <div class="resource-header">{} {} {}</div>"#, 
            css_class, symbol, 
            match resource.action {
                ActionType::Create => "CREATE",
                ActionType::Update => "UPDATE",
                ActionType::Destroy => "DESTROY",
                ActionType::Read => "READ",
                ActionType::NoOp => "NO-OP",
            },
            resource.id
        ));
        
        if !resource.attributes.is_empty() {
            html.push_str(r#"<div class="attributes">"#);
            for (key, value) in &resource.attributes {
                html.push_str(&format!("<div><strong>{}:</strong> {}</div>", key, value));
            }
            html.push_str(r#"</div>"#);
        }
        
        html.push_str(r#"</div>"#);
    }
    
    // HTML footer
    html.push_str(r#"
</body>
</html>"#);
    
    html
}