use crate::{Resource, ActionType};
use regex::Regex;
use std::collections::HashMap;

pub fn parse_resource_changes(lines: &[&str]) -> Vec<Resource> {
    let mut resources = Vec::new();
    let mut current_resource: Option<Resource> = None;
    
    for line in lines {
        // Detect resource action lines
        if let Some(resource) = parse_resource_action_line(line) {
            if let Some(existing) = current_resource.take() {
                resources.push(existing);
            }
            current_resource = Some(resource);
        }
        // TODO: Parse attribute changes within resources
    }
    
    if let Some(resource) = current_resource {
        resources.push(resource);
    }
    
    resources
}

fn parse_resource_action_line(line: &str) -> Option<Resource> {
    // Parse lines like "# aws_s3_bucket.example will be created"
    // The format is: resource_type.resource_name
    let create_regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_]+)\s+will be created").ok()?;
    let update_regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_]+)\s+will be updated").ok()?;
    let destroy_regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_]+)\s+will be destroyed").ok()?;
    
    if let Some(captures) = create_regex.captures(line) {
        return Some(create_resource_from_captures(&captures, ActionType::Create));
    }
    
    if let Some(captures) = update_regex.captures(line) {
        return Some(create_resource_from_captures(&captures, ActionType::Update));
    }
    
    if let Some(captures) = destroy_regex.captures(line) {
        return Some(create_resource_from_captures(&captures, ActionType::Destroy));
    }
    
    None
}

fn create_resource_from_captures(captures: &regex::Captures, action: ActionType) -> Resource {
    let resource_type = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown");
    let name = captures.get(2).map(|m| m.as_str()).unwrap_or("unknown");
    
    // Extract provider from resource type (e.g., aws_s3_bucket -> aws)
    let provider = resource_type.split('_').next().unwrap_or("unknown");
    
    Resource {
        id: format!("{}.{}", resource_type, name),
        name: name.to_string(),
        type_name: resource_type.to_string(),
        provider: provider.to_string(),
        action,
        changes: Vec::new(),
        attributes: HashMap::new(),
        applied: false,
    }
}