use crate::{Resource, ActionType};
use regex::Regex;
use std::collections::HashMap;
use serde_json::Value;
use lazy_static::lazy_static;

pub fn parse_resource_changes(lines: &[&str]) -> Vec<Resource> {
    let mut resources = Vec::new();
    let mut current_resource: Option<Resource> = None;
    let mut in_resource_block = false;
    let mut brace_level = 0;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Detect resource action lines
        if let Some(resource) = parse_resource_action_line(line) {
            if let Some(existing) = current_resource.take() {
                resources.push(existing);
            }
            current_resource = Some(resource);
            in_resource_block = false;
            brace_level = 0;
        }
        // Detect start of resource block (+ resource, ~ resource, - resource)
        else if trimmed.starts_with("+ resource") || 
                 trimmed.starts_with("~ resource") || 
                 trimmed.starts_with("- resource") {
            in_resource_block = true;
            brace_level = 0;
        }
        // Track brace levels to know when we're inside the resource
        else if in_resource_block {
            for ch in trimmed.chars() {
                match ch {
                    '{' => brace_level += 1,
                    '}' => {
                        brace_level -= 1;
                        if brace_level <= 0 {
                            in_resource_block = false;
                        }
                    },
                    _ => {}
                }
            }
            
            // Parse attribute changes within the resource block
            if brace_level > 0 {
                if let Some(ref mut resource) = current_resource {
                    parse_attribute_change(trimmed, &mut resource.attributes);
                }
            }
        }
    }
    
    if let Some(resource) = current_resource {
        resources.push(resource);
    }
    
    resources
}

lazy_static! {
    static ref CREATE_REGEX: Regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_-]+)\s+will be created").unwrap();
    static ref UPDATE_REGEX: Regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_-]+)\s+will be updated").unwrap();
    static ref DESTROY_REGEX: Regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_-]+)\s+will be destroyed").unwrap();
    static ref ARROW_CHANGE_REGEX: Regex = Regex::new(r"[~+-]?\s*(.+?)\s*=\s*(.+?)\s*->\s*(.+)").unwrap();
    static ref SIMPLE_ATTR_REGEX: Regex = Regex::new(r"([~+-]?)\s*(.+?)\s*=\s*(.+)").unwrap();
    static ref KNOWN_AFTER_APPLY_REGEX: Regex = Regex::new(r"[~+-]?\s*(.+?)\s*=").unwrap();
}

fn parse_resource_action_line(line: &str) -> Option<Resource> {
    // Parse lines like "# aws_s3_bucket.example will be created"
    // The format is: resource_type.resource_name
    
    if let Some(captures) = CREATE_REGEX.captures(line) {
        return Some(create_resource_from_captures(&captures, ActionType::Create));
    }
    
    if let Some(captures) = UPDATE_REGEX.captures(line) {
        return Some(create_resource_from_captures(&captures, ActionType::Update));
    }
    
    if let Some(captures) = DESTROY_REGEX.captures(line) {
        return Some(create_resource_from_captures(&captures, ActionType::Destroy));
    }
    
    None
}

fn create_resource_from_captures(captures: &regex::Captures, action: ActionType) -> Resource {
    let resource_type = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown");
    let resource_name = captures.get(2).map(|m| m.as_str()).unwrap_or("unknown");
    let resource_id = format!("{}.{}", resource_type, resource_name);
    
    Resource {
        id: resource_id,
        name: resource_name.to_string(),
        type_name: resource_type.to_string(),
        provider: "unknown".to_string(),
        action,
        changes: Vec::new(),
        attributes: HashMap::new(),
        applied: false,
    }
}

fn parse_attribute_change(line: &str, attributes: &mut HashMap<String, Value>) {
    let line = line.trim();
    
    // Parse different types of attribute changes:
    // + attribute = "value"           (addition)
    // - attribute = "value"           (removal)  
    // ~ attribute = "old" -> "new"    (change)
    //   attribute = "value"           (unchanged, for context)
    
    // Handle changes with -> arrow
    if let Some(change_match) = ARROW_CHANGE_REGEX.captures(line) {
        let attr_name = change_match.get(1).map(|m| m.as_str().trim()).unwrap_or("unknown");
        let old_value = change_match.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        let new_value = change_match.get(3).map(|m| m.as_str().trim()).unwrap_or("");
        
        attributes.insert(
            attr_name.to_string(),
            Value::String(format!("{} → {}", old_value, new_value))
        );
        return;
    }
    
    // Handle simple additions/removals/unchanged
    if let Some(attr_match) = SIMPLE_ATTR_REGEX.captures(line) {
        let change_type = attr_match.get(1).map(|m| m.as_str()).unwrap_or("");
        let attr_name = attr_match.get(2).map(|m| m.as_str().trim()).unwrap_or("unknown");
        let attr_value = attr_match.get(3).map(|m| m.as_str().trim()).unwrap_or("");
        
        let formatted_value = match change_type {
            "+" => format!("+ {}", attr_value),
            "-" => format!("- {}", attr_value),
            "~" => format!("~ {}", attr_value),
            _ => attr_value.to_string(),
        };
        
        attributes.insert(attr_name.to_string(), Value::String(formatted_value));
    }
    
    // Handle special cases like "(known after apply)"
    if line.contains("(known after apply)") {
        if let Some(attr_match) = KNOWN_AFTER_APPLY_REGEX.captures(line) {
            let attr_name = attr_match.get(1).map(|m| m.as_str().trim()).unwrap_or("unknown");
            attributes.insert(attr_name.to_string(), Value::String("(known after apply)".to_string()));
        }
    }
}