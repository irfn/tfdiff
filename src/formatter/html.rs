use crate::{TerraformPlan, ActionType};
use serde_json::Value;

pub fn format_html_output(plan: &TerraformPlan) -> String {
    let mut html = String::new();
    
    // HTML header with enhanced styling and JavaScript
    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Terraform Diff Report</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body { 
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        
        .header { 
            background: linear-gradient(135deg, #2c3e50 0%, #3498db 100%);
            color: white;
            padding: 40px 20px;
            text-align: center;
            position: relative;
        }
        
        .header h1 {
            font-size: 2.5em;
            font-weight: 300;
            margin-bottom: 10px;
        }
        
        .header::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 50%;
            transform: translateX(-50%);
            width: 100px;
            height: 3px;
            background: #ecf0f1;
            border-radius: 2px;
        }
        
        .summary { 
            background: #f8f9fa;
            padding: 30px 20px;
            border-bottom: 1px solid #e9ecef;
        }
        
        .summary h2 {
            color: #2c3e50;
            margin-bottom: 20px;
            font-size: 1.5em;
            text-align: center;
        }
        
        .summary-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-top: 20px;
        }
        
        .summary-item {
            background: white;
            padding: 20px;
            border-radius: 10px;
            text-align: center;
            box-shadow: 0 2px 10px rgba(0,0,0,0.05);
            transition: transform 0.2s ease;
        }
        
        .summary-item:hover {
            transform: translateY(-2px);
        }
        
        .summary-item .number {
            font-size: 2em;
            font-weight: bold;
            margin-bottom: 5px;
        }
        
        .summary-item .label {
            color: #666;
            font-size: 0.9em;
        }
        
        .add .number { color: #28a745; }
        .change .number { color: #ffc107; }
        .destroy .number { color: #dc3545; }
        .read .number { color: #17a2b8; }
        
        .filters {
            padding: 20px;
            background: #fff;
            border-bottom: 1px solid #e9ecef;
        }
        
        .filter-buttons {
            display: flex;
            gap: 10px;
            justify-content: center;
            flex-wrap: wrap;
        }
        
        .filter-btn {
            padding: 8px 16px;
            border: 2px solid #e9ecef;
            background: white;
            color: #666;
            border-radius: 20px;
            cursor: pointer;
            transition: all 0.2s ease;
            font-size: 0.9em;
        }
        
        .filter-btn:hover, .filter-btn.active {
            border-color: #3498db;
            color: #3498db;
            background: #f8f9fa;
        }
        
        .resources {
            padding: 20px;
        }
        
        .resource { 
            border: 1px solid #e9ecef;
            margin: 15px 0;
            border-radius: 10px;
            overflow: hidden;
            transition: all 0.2s ease;
            background: white;
        }
        
        .resource:hover {
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
            transform: translateY(-1px);
        }
        
        .resource-header { 
            padding: 20px;
            font-weight: 600;
            font-size: 1.1em;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 10px;
            transition: background-color 0.2s ease;
        }
        
        .resource-header:hover {
            background-color: rgba(0,0,0,0.02);
        }
        
        .resource-toggle {
            margin-left: auto;
            font-size: 1.2em;
            transition: transform 0.2s ease;
        }
        
        .resource.collapsed .resource-toggle {
            transform: rotate(-90deg);
        }
        
        .create { border-left: 5px solid #28a745; }
        .update { border-left: 5px solid #ffc107; }
        .destroy { border-left: 5px solid #dc3545; }
        .read { border-left: 5px solid #17a2b8; }
        .noop { border-left: 5px solid #6c757d; }
        
        .create .resource-header { color: #155724; }
        .update .resource-header { color: #856404; }
        .destroy .resource-header { color: #721c24; }
        .read .resource-header { color: #0c5460; }
        .noop .resource-header { color: #495057; }
        
        .attributes { 
            padding: 20px;
            background: #f8f9fa;
            border-top: 1px solid #e9ecef;
            transition: max-height 0.3s ease, padding 0.3s ease;
        }
        
        .resource.collapsed .attributes {
            max-height: 0;
            padding: 0 20px;
            overflow: hidden;
        }
        
        .attribute {
            margin: 10px 0;
            padding: 0;
            background: white;
            border-radius: 5px;
            border: 1px solid #e9ecef;
            overflow: hidden;
        }
        
        .attribute-header {
            padding: 8px 12px;
            background: #f8f9fa;
            border-bottom: 1px solid #e9ecef;
            font-weight: 600;
            color: #495057;
        }
        
        .attribute-content {
            padding: 12px;
        }
        
        .diff-container {
            display: flex;
            min-height: 40px;
        }
        
        .diff-side {
            flex: 1;
            padding: 8px 12px;
            font-family: 'Consolas', 'Monaco', monospace;
            font-size: 0.9em;
            line-height: 1.4;
            word-break: break-word;
        }
        
        .diff-side.before {
            background: #ffe6e6;
            border-right: 1px solid #ffcccc;
            color: #d73527;
        }
        
        .diff-side.after {
            background: #e6ffe6;
            color: #28a745;
        }
        
        .diff-side.unchanged {
            background: #f8f9fa;
            color: #495057;
            flex: 2;
        }
        
        .diff-side.addition {
            background: #e6ffe6;
            color: #28a745;
            flex: 2;
        }
        
        .diff-side.removal {
            background: #ffe6e6;
            color: #d73527;
            flex: 2;
        }
        
        .diff-arrow {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 40px;
            background: #f8f9fa;
            color: #6c757d;
            border-left: 1px solid #e9ecef;
            border-right: 1px solid #e9ecef;
        }
        
        .known-after-apply {
            font-style: italic;
            color: #6c757d;
            background: #f8f9fa !important;
        }
        
        .footer {
            text-align: center;
            padding: 30px 20px;
            background: #f8f9fa;
            color: #666;
            font-size: 0.9em;
        }
        
        .search-box {
            width: 100%;
            max-width: 400px;
            padding: 10px 15px;
            border: 2px solid #e9ecef;
            border-radius: 25px;
            font-size: 1em;
            margin: 0 auto 20px;
            display: block;
            transition: border-color 0.2s ease;
        }
        
        .search-box:focus {
            outline: none;
            border-color: #3498db;
        }
        
        .hidden {
            display: none !important;
        }
        
        @media (max-width: 768px) {
            .header h1 {
                font-size: 1.8em;
            }
            
            .summary-grid {
                grid-template-columns: repeat(2, 1fr);
            }
            
            .filter-buttons {
                justify-content: flex-start;
            }
            
            .resource-header {
                font-size: 1em;
                padding: 15px;
            }
        }
    </style>
</head>
<body>
    <div class="container">"#);
    
    // Title
    html.push_str(&format!(r#"
        <div class="header">
            <h1>🌊 Terraform {} Report</h1>
            <p>Generated at {}</p>
        </div>"#, 
        match plan.mode {
            crate::PlanMode::Plan => "Plan",
            crate::PlanMode::Apply => "Apply",
        },
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));
    
    // Summary
    html.push_str(r#"<div class="summary">
        <h2>📊 Summary</h2>
        <div class="summary-grid">"#);
    
    if plan.summary.add > 0 {
        html.push_str(&format!(r#"
            <div class="summary-item add">
                <div class="number">✚ {}</div>
                <div class="label">to add</div>
            </div>"#, plan.summary.add));
    }
    
    if plan.summary.change > 0 {
        html.push_str(&format!(r#"
            <div class="summary-item change">
                <div class="number">↻ {}</div>
                <div class="label">to change</div>
            </div>"#, plan.summary.change));
    }
    
    if plan.summary.destroy > 0 {
        html.push_str(&format!(r#"
            <div class="summary-item destroy">
                <div class="number">✖ {}</div>
                <div class="label">to destroy</div>
            </div>"#, plan.summary.destroy));
    }
    
    if plan.summary.read > 0 {
        html.push_str(&format!(r#"
            <div class="summary-item read">
                <div class="number">⇐ {}</div>
                <div class="label">to read</div>
            </div>"#, plan.summary.read));
    }
    
    html.push_str(r#"
        </div>
    </div>"#);
    
    // Filters and search
    html.push_str(r#"
    <div class="filters">
        <input type="text" class="search-box" placeholder="🔍 Search resources..." id="searchBox">
        <div class="filter-buttons">
            <button class="filter-btn active" data-filter="all">All Resources</button>
            <button class="filter-btn" data-filter="create">✚ Create</button>
            <button class="filter-btn" data-filter="update">↻ Update</button>
            <button class="filter-btn" data-filter="destroy">✖ Destroy</button>
            <button class="filter-btn" data-filter="read">⇐ Read</button>
        </div>
    </div>"#);
    
    // Resources
    html.push_str(r#"<div class="resources">"#);
    
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
        
        let action_text = match resource.action {
            ActionType::Create => "CREATE",
            ActionType::Update => "UPDATE",
            ActionType::Destroy => "DESTROY",
            ActionType::Read => "READ",
            ActionType::NoOp => "NO-OP",
        };
        
        html.push_str(&format!(r#"
        <div class="resource {}" data-action="{}">
            <div class="resource-header" onclick="toggleResource(this)">
                <span>{} {} {}</span>
                <span class="resource-toggle">▼</span>
            </div>"#, 
            css_class, css_class, symbol, action_text, resource.id
        ));
        
        if !resource.attributes.is_empty() {
            html.push_str(r#"<div class="attributes">"#);
            for (key, value) in &resource.attributes {
                html.push_str(&format!(r#"<div class="attribute">
                    <div class="attribute-header">{}</div>
                    <div class="attribute-content">{}</div>
                </div>"#, key, format_attribute_diff(value)));
            }
            html.push_str(r#"</div>"#);
        }
        
        html.push_str(r#"</div>"#);
    }
    
    html.push_str(r#"</div>"#);
    
    // Footer
    html.push_str(r#"
        <div class="footer">
            <p>Generated by <strong>tfdiff</strong> - Beautiful Terraform plan and apply output formatter</p>
        </div>
    </div>
    
    <script>
        // Resource toggling
        function toggleResource(header) {
            const resource = header.parentElement;
            resource.classList.toggle('collapsed');
        }
        
        // Filter functionality
        document.querySelectorAll('.filter-btn').forEach(btn => {
            btn.addEventListener('click', function() {
                // Update active button
                document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
                this.classList.add('active');
                
                const filter = this.getAttribute('data-filter');
                const resources = document.querySelectorAll('.resource');
                
                resources.forEach(resource => {
                    if (filter === 'all' || resource.getAttribute('data-action') === filter) {
                        resource.style.display = 'block';
                    } else {
                        resource.style.display = 'none';
                    }
                });
            });
        });
        
        // Search functionality
        document.getElementById('searchBox').addEventListener('input', function(e) {
            const searchTerm = e.target.value.toLowerCase();
            const resources = document.querySelectorAll('.resource');
            
            resources.forEach(resource => {
                const text = resource.textContent.toLowerCase();
                if (text.includes(searchTerm)) {
                    resource.style.display = 'block';
                } else {
                    resource.style.display = 'none';
                }
            });
        });
        
        // Keyboard shortcuts
        document.addEventListener('keydown', function(e) {
            if (e.key === '/') {
                e.preventDefault();
                document.getElementById('searchBox').focus();
            }
            
            if (e.key === 'Escape') {
                document.getElementById('searchBox').blur();
                document.getElementById('searchBox').value = '';
                document.getElementById('searchBox').dispatchEvent(new Event('input'));
            }
        });
        
        // Initialize - collapse resources by default on mobile
        if (window.innerWidth <= 768) {
            document.querySelectorAll('.resource').forEach(resource => {
                resource.classList.add('collapsed');
            });
        }
    </script>
</body>
</html>"#);
    
    html
}

fn format_attribute_diff(value: &Value) -> String {
    let value_str = match value {
        Value::String(s) => s.clone(),
        _ => value.to_string(),
    };
    
    // Handle known after apply
    if value_str.contains("(known after apply)") {
        return format!(r#"<div class="diff-container">
            <div class="diff-side known-after-apply">{}</div>
        </div>"#, value_str);
    }
    
    // Handle changes with arrow (old → new)
    if value_str.contains(" → ") {
        let parts: Vec<&str> = value_str.splitn(2, " → ").collect();
        if parts.len() == 2 {
            return format!(r#"<div class="diff-container">
                <div class="diff-side before">{}</div>
                <div class="diff-arrow">→</div>
                <div class="diff-side after">{}</div>
            </div>"#, 
            html_escape(parts[0]), 
            html_escape(parts[1]));
        }
    }
    
    // Handle additions (+ value)
    if value_str.starts_with("+ ") {
        return format!(r#"<div class="diff-container">
            <div class="diff-side addition">{}</div>
        </div>"#, html_escape(&value_str[2..]));
    }
    
    // Handle removals (- value)  
    if value_str.starts_with("- ") {
        return format!(r#"<div class="diff-container">
            <div class="diff-side removal">{}</div>
        </div>"#, html_escape(&value_str[2..]));
    }
    
    // Handle modifications (~ value)
    if value_str.starts_with("~ ") {
        return format!(r#"<div class="diff-container">
            <div class="diff-side unchanged">Modified: {}</div>
        </div>"#, html_escape(&value_str[2..]));
    }
    
    // Default unchanged value
    format!(r#"<div class="diff-container">
        <div class="diff-side unchanged">{}</div>
    </div>"#, html_escape(&value_str))
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#x27;")
}