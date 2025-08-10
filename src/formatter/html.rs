use crate::{TerraformPlan, ActionType};
use serde_json::{Value, json};
use chrono::Local;

pub fn format_html_output(plan: &TerraformPlan) -> String {
    let mut html = String::new();
    
    // HTML header - exactly matching the sample
    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Terraform Plan - Infrastructure</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            background-color: #0d1117;
            color: #c9d1d9;
            line-height: 1.6;
            padding: 20px;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
        }
        
        /* Header Section */
        .header {
            text-align: center;
            margin-bottom: 30px;
            padding: 20px;
            background: linear-gradient(135deg, #1e3a5f 0%, #0d1117 100%);
            border-radius: 12px;
            border: 1px solid #30363d;
            position: relative;
            overflow: hidden;
        }
        
        .header::before {
            content: '';
            position: absolute;
            top: -50%;
            left: -50%;
            width: 200%;
            height: 200%;
            background: linear-gradient(45deg, transparent, rgba(88, 166, 255, 0.1), transparent);
            transform: rotate(45deg);
            animation: shimmer 3s linear infinite;
        }
        
        @keyframes shimmer {
            0% { transform: translateX(-100%) translateY(-100%) rotate(45deg); }
            100% { transform: translateX(100%) translateY(100%) rotate(45deg); }
        }
        
        .header h1 {
            color: #58a6ff;
            font-size: 28px;
            margin-bottom: 10px;
            text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
            position: relative;
            z-index: 1;
        }
        
        .header .subtitle {
            color: #8b949e;
            font-size: 14px;
            position: relative;
            z-index: 1;
        }
        
        .timestamp {
            color: #6e7681;
            font-size: 12px;
            margin-top: 10px;
            position: relative;
            z-index: 1;
        }
        
        /* Summary Card */
        .summary-card {
            background-color: #161b22;
            border: 1px solid #30363d;
            border-radius: 12px;
            padding: 30px;
            margin-bottom: 30px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        
        .summary-title {
            color: #58a6ff;
            font-size: 20px;
            margin-bottom: 20px;
            display: flex;
            align-items: center;
            gap: 10px;
        }
        
        .summary-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
        }
        
        .stat-box {
            background-color: #0d1117;
            border: 1px solid #30363d;
            border-radius: 8px;
            padding: 20px;
            text-align: center;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
        }
        
        .stat-box::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 3px;
            background: currentColor;
            transform: scaleX(0);
            transition: transform 0.3s ease;
        }
        
        .stat-box:hover {
            transform: translateY(-3px);
            box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
        }
        
        .stat-box:hover::before {
            transform: scaleX(1);
        }
        
        .stat-icon {
            font-size: 32px;
            margin-bottom: 10px;
        }
        
        .stat-number {
            font-size: 36px;
            font-weight: bold;
            margin-bottom: 5px;
        }
        
        .stat-label {
            font-size: 14px;
            color: #8b949e;
        }
        
        .add { color: #3fb950; }
        .add .stat-number { color: #3fb950; }
        
        .change { color: #d29922; }
        .change .stat-number { color: #d29922; }
        
        .destroy { color: #f85149; }
        .destroy .stat-number { color: #f85149; }
        
        .read { color: #58a6ff; }
        .read .stat-number { color: #58a6ff; }
        
        /* Resource Sections */
        .resource-section {
            background-color: #161b22;
            border: 1px solid #30363d;
            border-radius: 12px;
            margin-bottom: 20px;
            overflow: hidden;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            transition: box-shadow 0.3s ease;
        }
        
        .resource-section:hover {
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        }
        
        .resource-header {
            background-color: #21262d;
            padding: 20px;
            border-bottom: 1px solid #30363d;
            display: flex;
            align-items: center;
            gap: 15px;
        }
        
        .action-badge {
            padding: 6px 12px;
            border-radius: 6px;
            font-size: 12px;
            font-weight: bold;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        }
        
        .action-create { 
            background-color: #3fb950; 
            color: #0d1117;
            animation: pulse-green 2s infinite;
        }
        
        @keyframes pulse-green {
            0%, 100% { box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2); }
            50% { box-shadow: 0 2px 8px rgba(63, 185, 80, 0.4); }
        }
        
        .action-update { background-color: #d29922; color: #0d1117; }
        .action-destroy { background-color: #f85149; color: #ffffff; }
        .action-read { background-color: #58a6ff; color: #0d1117; }
        
        .resource-name {
            color: #f0f6fc;
            font-size: 16px;
            font-weight: 600;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
        }
        
        .resource-id {
            color: #8b949e;
            font-size: 14px;
            margin-left: auto;
        }
        
        /* Diff Container */
        .diff-container {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 2px;
            background-color: #30363d;
        }
        
        .diff-side {
            background-color: #0d1117;
            padding: 20px;
            overflow-x: auto;
        }
        
        .diff-side.single {
            grid-column: 1 / -1;
        }
        
        .diff-header {
            color: #58a6ff;
            font-size: 12px;
            font-weight: bold;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: 15px;
            padding-bottom: 10px;
            border-bottom: 1px solid #30363d;
        }
        
        .diff-content {
            font-size: 13px;
            white-space: pre-wrap;
            word-wrap: break-word;
            line-height: 1.8;
        }
        
        .line {
            padding: 2px 0;
            margin: 2px 0;
            transition: background-color 0.2s ease;
        }
        
        .line.add {
            background-color: rgba(63, 185, 80, 0.1);
            color: #3fb950;
            padding-left: 20px;
            position: relative;
            border-left: 3px solid #3fb950;
        }
        
        .line.add::before {
            content: '+';
            position: absolute;
            left: 5px;
            font-weight: bold;
        }
        
        .line.remove {
            background-color: rgba(248, 81, 73, 0.1);
            color: #f85149;
            padding-left: 20px;
            position: relative;
            border-left: 3px solid #f85149;
        }
        
        .line.remove::before {
            content: '-';
            position: absolute;
            left: 5px;
            font-weight: bold;
        }
        
        .line.unchanged {
            color: #8b949e;
            font-style: italic;
        }
        
        .attribute {
            color: #79c0ff;
            font-weight: 600;
        }
        
        .value {
            color: #a5d6ff;
        }
        
        .string-value {
            color: #79c0ff;
        }
        
        /* Special Resource Highlight */
        .new-resource {
            position: relative;
        }
        
        .new-resource::before {
            content: 'NEW';
            position: absolute;
            top: 10px;
            right: 10px;
            background-color: #3fb950;
            color: #0d1117;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 10px;
            font-weight: bold;
            letter-spacing: 1px;
            animation: blink 2s infinite;
        }
        
        @keyframes blink {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.7; }
        }
        
        /* Warnings Section */
        .warnings-section {
            background-color: #161b22;
            border: 1px solid #8b5a00;
            border-radius: 12px;
            padding: 20px;
            margin-top: 30px;
        }
        
        .warnings-header {
            color: #d29922;
            font-size: 18px;
            margin-bottom: 15px;
            display: flex;
            align-items: center;
            gap: 10px;
        }
        
        .warning-item {
            background-color: rgba(210, 153, 34, 0.1);
            border-left: 3px solid #d29922;
            padding: 15px;
            margin-bottom: 10px;
            border-radius: 4px;
        }
        
        .warning-title {
            color: #d29922;
            font-weight: bold;
            margin-bottom: 5px;
        }
        
        .warning-text {
            color: #c9d1d9;
            font-size: 13px;
        }
        
        /* Code block styling */
        .code-block {
            background-color: #161b22;
            border: 1px solid #30363d;
            border-radius: 6px;
            padding: 15px;
            margin: 10px 0;
            overflow-x: auto;
        }
        
        @media (max-width: 768px) {
            .diff-container {
                grid-template-columns: 1fr;
            }
            
            .summary-grid {
                grid-template-columns: 1fr 1fr;
            }
        }
    </style>
</head>
<body>
    <div class="container">"#);
    
    // Header with shimmer effect
    let timestamp = Local::now().format("%B %d, %Y").to_string();
    html.push_str(&format!(r#"
        <div class="header">
            <h1>Terraform Plan Analysis</h1>
            <div class="subtitle">Infrastructure changes</div>
            <div class="timestamp">Generated on {}</div>
        </div>"#, timestamp));
    
    // Summary Card
    html.push_str(r#"
        <div class="summary-card">
            <div class="summary-title">
                <span>📊</span>
                <span>Plan Summary</span>
            </div>
            <div class="summary-grid">"#);
    
    if plan.summary.add > 0 {
        html.push_str(&format!(r#"
                <div class="stat-box add">
                    <div class="stat-icon">➕</div>
                    <div class="stat-number">{}</div>
                    <div class="stat-label">to add</div>
                </div>"#, plan.summary.add));
    }
    
    if plan.summary.change > 0 {
        html.push_str(&format!(r#"
                <div class="stat-box change">
                    <div class="stat-icon">🔄</div>
                    <div class="stat-number">{}</div>
                    <div class="stat-label">to change</div>
                </div>"#, plan.summary.change));
    }
    
    if plan.summary.destroy > 0 {
        html.push_str(&format!(r#"
                <div class="stat-box destroy">
                    <div class="stat-icon">🗑️</div>
                    <div class="stat-number">{}</div>
                    <div class="stat-label">to destroy</div>
                </div>"#, plan.summary.destroy));
    }
    
    if plan.summary.read > 0 || !plan.data_sources.is_empty() {
        let read_count = if plan.summary.read > 0 { 
            plan.summary.read 
        } else { 
            plan.data_sources.len() 
        };
        html.push_str(&format!(r#"
                <div class="stat-box read">
                    <div class="stat-icon">📖</div>
                    <div class="stat-number">{}</div>
                    <div class="stat-label">data source{} to read</div>
                </div>"#, 
                read_count,
                if read_count == 1 { "" } else { "s" }
        ));
    }
    
    html.push_str(r#"
            </div>
        </div>"#);
    
    // Resources
    for resource in &plan.resources {
        let (action_class, action_text) = match resource.action {
            ActionType::Create => ("action-create", "CREATE"),
            ActionType::Update => ("action-update", "UPDATE"),
            ActionType::Destroy => ("action-destroy", "DESTROY"),
            ActionType::Read => ("action-read", "READ"),
            ActionType::NoOp => ("action-read", "NO-OP"),
        };
        
        let new_resource_class = if resource.action == ActionType::Create { 
            " new-resource" 
        } else { 
            "" 
        };
        
        html.push_str(&format!(r#"
        <div class="resource-section{}">
            <div class="resource-header">
                <span class="action-badge {}">{}</span>
                <span class="resource-name">{}</span>"#,
            new_resource_class, action_class, action_text, resource.id));
        
        // Add resource ID if it's an update
        if resource.action == ActionType::Update && !resource.attributes.is_empty() {
            if let Some(id_value) = resource.attributes.get("id") {
                if let Value::String(id_str) = id_value {
                    html.push_str(&format!(r#"
                <span class="resource-id">id: {}</span>"#, id_str));
                }
            }
        }
        
        html.push_str(r#"
            </div>"#);
        
        // Format resource content based on action type
        match resource.action {
            ActionType::Update => {
                // For updates, show side-by-side diff
                html.push_str(&format_update_diff(resource));
            },
            _ => {
                // For create/destroy/read, show single column
                html.push_str(&format_single_column(resource));
            }
        }
        
        html.push_str(r#"
        </div>"#);
    }
    
    // Data Sources
    for data_source in &plan.data_sources {
        html.push_str(&format!(r#"
        <div class="resource-section">
            <div class="resource-header">
                <span class="action-badge action-read">READ</span>
                <span class="resource-name">{}</span>
            </div>"#, data_source.id));
        
        html.push_str(r#"
            <div class="diff-container">
                <div class="diff-side single">
                    <div class="diff-content">"#);
        
        // Handle data source attributes
        for (key, value) in &data_source.attributes {
            html.push_str(&format_attribute_line(key, value, LineType::Add));
        }
        
        html.push_str(r#"</div>
                </div>
            </div>
        </div>"#);
    }
    
    // Warnings Section
    if !plan.warnings.is_empty() {
        html.push_str(r#"
        <div class="warnings-section">
            <div class="warnings-header">
                <span>⚠️</span>
                <span>Warnings</span>
            </div>"#);
        
        for warning in &plan.warnings {
            html.push_str(&format!(r#"
            <div class="warning-item">
                <div class="warning-title">Warning</div>
                <div class="warning-text">{}</div>
            </div>"#, html_escape(&warning.message)));
        }
        
        html.push_str(r#"
        </div>"#);
    }
    
    html.push_str(r#"
    </div>
</body>
</html>"#);
    
    html
}

enum LineType {
    Add,
    Remove,
    Unchanged,
}

fn format_update_diff(resource: &crate::Resource) -> String {
    let mut html = String::new();
    
    // If we have changes, show them in side-by-side format
    if !resource.changes.is_empty() {
        // Group changes for before/after display
        let mut before_lines = Vec::new();
        let mut after_lines = Vec::new();
        
        for change in &resource.changes {
            let path = change.path.join(".");
            
            match (&change.before, &change.after) {
                (Some(before), Some(after)) if before != after => {
                    // Changed value
                    before_lines.push(format!(r#"<span class="line remove">    <span class="attribute">{}</span> = {}</span>"#,
                        path, format_value(before, false)));
                    after_lines.push(format!(r#"<span class="line add">    <span class="attribute">{}</span> = {}</span>"#,
                        path, format_value(after, false)));
                },
                (None, Some(after)) => {
                    // Added value
                    after_lines.push(format!(r#"<span class="line add">    <span class="attribute">{}</span> = {}</span>"#,
                        path, format_value(after, false)));
                },
                (Some(before), None) => {
                    // Removed value
                    before_lines.push(format!(r#"<span class="line remove">    <span class="attribute">{}</span> = {}</span>"#,
                        path, format_value(before, false)));
                },
                _ => {
                    // Unchanged or computed
                    if change.computed {
                        let line = format!(r#"<span class="line unchanged">    <span class="attribute">{}</span> = <span class="value">(known after apply)</span></span>"#, path);
                        before_lines.push(line.clone());
                        after_lines.push(line);
                    }
                }
            }
        }
        
        html.push_str(r#"
            <div class="diff-container">
                <div class="diff-side">
                    <div class="diff-header">Current State</div>
                    <div class="diff-content">"#);
        
        if before_lines.is_empty() {
            html.push_str(r#"<span class="line unchanged">    # (no previous state)</span>"#);
        } else {
            for line in before_lines {
                html.push_str(&line);
                html.push('\n');
            }
        }
        
        html.push_str(r#"</div>
                </div>
                <div class="diff-side">
                    <div class="diff-header">After Apply</div>
                    <div class="diff-content">"#);
        
        for line in after_lines {
            html.push_str(&line);
            html.push('\n');
        }
        
        html.push_str(r#"</div>
                </div>
            </div>"#);
    } else if !resource.attributes.is_empty() {
        // Fallback to parsing attributes for side-by-side display
        html.push_str(&format_attributes_as_diff(resource));
    }
    
    html
}

fn format_attributes_as_diff(resource: &crate::Resource) -> String {
    let mut html = String::new();
    let mut before_lines = Vec::new();
    let mut after_lines = Vec::new();
    
    // Parse attributes and convert arrow notation to side-by-side
    for (key, value) in &resource.attributes {
        if let Value::String(val_str) = value {
            // Check for arrow notation (old → new)
            if val_str.contains(" → ") {
                let parts: Vec<&str> = val_str.splitn(2, " → ").collect();
                if parts.len() == 2 {
                    before_lines.push(format!(r#"<span class="line remove">    <span class="attribute">{}</span> = {}</span>"#,
                        key, format_value(&json!(parts[0]), true)));
                    after_lines.push(format!(r#"<span class="line add">    <span class="attribute">{}</span> = {}</span>"#,
                        key, format_value(&json!(parts[1]), true)));
                    continue;
                }
            }
            // Check for addition prefix
            else if val_str.starts_with("+ ") {
                after_lines.push(format!(r#"<span class="line add">    <span class="attribute">{}</span> = {}</span>"#,
                    key, format_value(&json!(&val_str[2..]), true)));
                continue;
            }
            // Check for removal prefix
            else if val_str.starts_with("- ") {
                before_lines.push(format!(r#"<span class="line remove">    <span class="attribute">{}</span> = {}</span>"#,
                    key, format_value(&json!(&val_str[2..]), true)));
                continue;
            }
        }
        
        // Default: show on both sides as unchanged
        let line = format!(r#"<span class="line unchanged">    <span class="attribute">{}</span> = {}</span>"#,
            key, format_value(value, true));
        before_lines.push(line.clone());
        after_lines.push(line);
    }
    
    html.push_str(r#"
            <div class="diff-container">
                <div class="diff-side">
                    <div class="diff-header">Current State</div>
                    <div class="diff-content">"#);
    
    if before_lines.is_empty() {
        html.push_str(r#"<span class="line unchanged">    # (no previous state)</span>"#);
    } else {
        for line in before_lines {
            html.push_str(&line);
            html.push('\n');
        }
    }
    
    html.push_str(r#"</div>
                </div>
                <div class="diff-side">
                    <div class="diff-header">After Apply</div>
                    <div class="diff-content">"#);
    
    for line in after_lines {
        html.push_str(&line);
        html.push('\n');
    }
    
    html.push_str(r#"</div>
                </div>
            </div>"#);
    
    html
}

fn format_single_column(resource: &crate::Resource) -> String {
    let mut html = String::new();
    
    html.push_str(r#"
            <div class="diff-container">
                <div class="diff-side single">
                    <div class="diff-content">"#);
    
    // Show changes if available
    if !resource.changes.is_empty() {
        for change in &resource.changes {
            let path = change.path.join(".");
            
            match (&change.before, &change.after) {
                (None, Some(after)) => {
                    html.push_str(&format!(r#"<span class="line add"><span class="attribute">{}</span> = {}</span>
"#, path, format_value(after, true)));
                },
                (Some(before), None) => {
                    html.push_str(&format!(r#"<span class="line remove"><span class="attribute">{}</span> = {}</span>
"#, path, format_value(before, true)));
                },
                _ => {
                    if change.computed {
                        html.push_str(&format!(r#"<span class="line add"><span class="attribute">{}</span> = <span class="value">(known after apply)</span></span>
"#, path));
                    }
                }
            }
        }
    } else {
        // Show attributes
        for (key, value) in &resource.attributes {
            let line_type = match resource.action {
                ActionType::Create => LineType::Add,
                ActionType::Destroy => LineType::Remove,
                _ => LineType::Unchanged,
            };
            html.push_str(&format_attribute_line(key, value, line_type));
        }
    }
    
    html.push_str(r#"</div>
                </div>
            </div>"#);
    
    html
}

fn format_attribute_line(key: &str, value: &Value, line_type: LineType) -> String {
    let class = match line_type {
        LineType::Add => "add",
        LineType::Remove => "remove",
        LineType::Unchanged => "unchanged",
    };
    
    format!(r#"<span class="line {}"><span class="attribute">{}</span> = {}</span>
"#, class, key, format_value(value, true))
}

fn format_value(value: &Value, with_quotes: bool) -> String {
    match value {
        Value::String(s) => {
            if s == "(known after apply)" {
                format!(r#"<span class="value">{}</span>"#, s)
            } else if with_quotes {
                format!(r#"<span class="string-value">"{}"</span>"#, html_escape(s))
            } else {
                format!(r#"<span class="string-value">{}</span>"#, html_escape(s))
            }
        },
        Value::Number(n) => format!(r#"<span class="value">{}</span>"#, n),
        Value::Bool(b) => format!(r#"<span class="value">{}</span>"#, b),
        Value::Null => format!(r#"<span class="value">null</span>"#),
        Value::Array(arr) => {
            if arr.is_empty() {
                format!(r#"<span class="value">[]</span>"#)
            } else {
                let items: Vec<String> = arr.iter()
                    .map(|v| format_value(v, true))
                    .collect();
                format!(r#"[
{}
]"#, items.join(",\n    "))
            }
        },
        Value::Object(obj) => {
            if obj.is_empty() {
                format!(r#"<span class="value">{{}}</span>"#)
            } else {
                let mut items = Vec::new();
                for (k, v) in obj {
                    items.push(format!(r#"    <span class="attribute">{}</span> = {}"#, 
                        k, format_value(v, true)));
                }
                format!(r#"{{
{}
}}"#, items.join("\n"))
            }
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#x27;")
}