use crate::{ActionType, Change, Resource};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn parse_resource_changes(lines: &[&str]) -> Vec<Resource> {
    let mut resources = Vec::new();
    let mut current_resource: Option<Resource> = None;
    let mut in_resource_block = false;
    let mut brace_level = 0;

    // Multi-line array collection state
    let mut collecting_multiline = false;
    let mut multiline_attr_name = String::new();
    let mut multiline_change_type = String::new();
    let mut multiline_lines: Vec<String> = Vec::new();
    let mut multiline_bracket_depth: i32 = 0;

    for line in lines {
        let trimmed = line.trim();

        // Detect resource action lines
        if let Some(resource) = parse_resource_action_line(line) {
            // Flush any pending multiline collection
            if collecting_multiline {
                if let Some(ref mut res) = current_resource {
                    flush_multiline_array_change(
                        &multiline_attr_name,
                        &multiline_change_type,
                        &multiline_lines,
                        &mut res.changes,
                    );
                }
                collecting_multiline = false;
                multiline_lines.clear();
            }

            if let Some(existing) = current_resource.take() {
                resources.push(existing);
            }
            current_resource = Some(resource);
            in_resource_block = false;
            brace_level = 0;
        }
        // Detect start of resource block (+ resource, ~ resource, - resource)
        else if current_resource.is_some()
            && (trimmed.starts_with("+ resource")
                || trimmed.starts_with("~ resource")
                || trimmed.starts_with("- resource"))
        {
            in_resource_block = true;
            brace_level = 0;
            // Count braces on this line too
            for ch in trimmed.chars() {
                match ch {
                    '{' => brace_level += 1,
                    '}' => brace_level -= 1,
                    _ => {}
                }
            }
        }
        // Track brace levels to know when we're inside the resource
        else if in_resource_block {
            // If we're collecting a multiline array, continue collecting
            if collecting_multiline {
                // Track bracket depth (only square brackets for arrays)
                for ch in trimmed.chars() {
                    match ch {
                        '[' => multiline_bracket_depth += 1,
                        ']' => multiline_bracket_depth -= 1,
                        _ => {}
                    }
                }

                multiline_lines.push(trimmed.to_string());

                if multiline_bracket_depth <= 0 {
                    // Collection complete - flush the multiline change
                    if let Some(ref mut resource) = current_resource {
                        flush_multiline_array_change(
                            &multiline_attr_name,
                            &multiline_change_type,
                            &multiline_lines,
                            &mut resource.changes,
                        );
                    }
                    collecting_multiline = false;
                    multiline_lines.clear();
                }

                continue; // Skip regular processing for multiline content
            }

            for ch in trimmed.chars() {
                match ch {
                    '{' => brace_level += 1,
                    '}' => {
                        brace_level -= 1;
                        if brace_level <= 0 {
                            in_resource_block = false;
                        }
                    }
                    _ => {}
                }
            }

            // Parse attribute changes within the resource block
            if brace_level > 0 {
                if let Some(ref mut resource) = current_resource {
                    if resource.action == ActionType::Update {
                        // Check if this line starts a multiline array
                        if let Some((change_type, attr_name)) =
                            detect_multiline_array_start(trimmed)
                        {
                            collecting_multiline = true;
                            multiline_attr_name = attr_name;
                            multiline_change_type = change_type;
                            multiline_lines.clear();
                            multiline_bracket_depth = 1; // We've seen the opening `[`
                        } else {
                            // For updates, parse changes with before/after values
                            parse_update_change(trimmed, &mut resource.changes);
                        }
                    } else {
                        // For create/destroy, just parse attributes
                        parse_attribute_change(trimmed, &mut resource.attributes);
                    }
                }
            }
        }
    }

    // Flush any remaining multiline collection
    if collecting_multiline {
        if let Some(ref mut res) = current_resource {
            flush_multiline_array_change(
                &multiline_attr_name,
                &multiline_change_type,
                &multiline_lines,
                &mut res.changes,
            );
        }
    }

    if let Some(resource) = current_resource {
        resources.push(resource);
    }

    resources
}

lazy_static! {
    static ref CREATE_REGEX: Regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_\-\.]+(?:_[A-F0-9]+)?)\s+(?:\([^)]+\)\s+)?will be created").unwrap();
    static ref UPDATE_REGEX: Regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_\-\.]+(?:_[A-F0-9]+)?)\s+(?:\([^)]+\)\s+)?will be (?:updated|updated in-place)").unwrap();
    static ref DESTROY_REGEX: Regex = Regex::new(r"#\s+([a-zA-Z0-9_]+)\.([a-zA-Z0-9_\-\.]+(?:_[A-F0-9]+)?)\s+(?:\([^)]+\)\s+)?will be destroyed").unwrap();
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
        return Some(create_resource_from_captures(
            &captures,
            ActionType::Destroy,
        ));
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
        let attr_name = change_match
            .get(1)
            .map(|m| m.as_str().trim())
            .unwrap_or("unknown");
        let old_value = change_match.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        let new_value = change_match.get(3).map(|m| m.as_str().trim()).unwrap_or("");

        attributes.insert(
            attr_name.to_string(),
            Value::String(format!("{} → {}", old_value, new_value)),
        );
        return;
    }

    // Handle simple additions/removals/unchanged
    if let Some(attr_match) = SIMPLE_ATTR_REGEX.captures(line) {
        let change_type = attr_match.get(1).map(|m| m.as_str()).unwrap_or("");
        let attr_name = attr_match
            .get(2)
            .map(|m| m.as_str().trim())
            .unwrap_or("unknown");
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
            let attr_name = attr_match
                .get(1)
                .map(|m| m.as_str().trim())
                .unwrap_or("unknown");
            attributes.insert(
                attr_name.to_string(),
                Value::String("(known after apply)".to_string()),
            );
        }
    }
}

fn parse_update_change(line: &str, changes: &mut Vec<Change>) {
    let line = line.trim();

    // Skip empty lines, comments, and structural lines
    if line.is_empty() || line.starts_with('#') || line == "{" || line == "}" {
        return;
    }

    // Parse different types of attribute changes for updates:
    // ~ attribute = "old" -> "new"    (modification)
    // + attribute = "value"           (addition)
    // - attribute = "value"           (removal)

    // Extract the attribute path (handle nested attributes)
    let mut path = Vec::new();

    // Remove leading change indicator; skip if not a change line
    let Some((change_type, clean_line)) = split_change_indicator(line) else {
        return;
    };

    // Handle arrow changes (old -> new)
    if clean_line.contains(" -> ") && clean_line.contains(" = ") {
        if let Some(eq_pos) = clean_line.find(" = ") {
            let attr_name = clean_line[..eq_pos].trim();
            let values_part = &clean_line[eq_pos + 3..];

            if let Some(arrow_pos) = values_part.find(" -> ") {
                let old_value = values_part[..arrow_pos].trim();
                let new_value = values_part[arrow_pos + 4..].trim();

                path.push(attr_name.to_string());

                let change = Change {
                    path: path.clone(),
                    before: Some(parse_terraform_value(old_value)),
                    after: Some(parse_terraform_value(new_value)),
                    sensitive: false,
                    computed: false,
                };

                changes.push(change);
            }
        }
    }
    // Handle simple additions
    else if change_type == "+" && clean_line.contains(" = ") {
        if let Some(eq_pos) = clean_line.find(" = ") {
            let attr_name = clean_line[..eq_pos].trim();
            let value = clean_line[eq_pos + 3..].trim();

            path.push(attr_name.to_string());

            let change = Change {
                path: path.clone(),
                before: None,
                after: Some(parse_terraform_value(value)),
                sensitive: false,
                computed: value == "(known after apply)",
            };

            changes.push(change);
        }
    }
    // Handle simple removals
    else if change_type == "-" && clean_line.contains(" = ") {
        if let Some(eq_pos) = clean_line.find(" = ") {
            let attr_name = clean_line[..eq_pos].trim();
            let value = clean_line[eq_pos + 3..].trim();

            path.push(attr_name.to_string());

            let change = Change {
                path: path.clone(),
                before: Some(parse_terraform_value(value)),
                after: None,
                sensitive: false,
                computed: false,
            };

            changes.push(change);
        }
    }
}

/// Split a leading "~ ", "+ " or "- " change indicator from a diff line.
fn split_change_indicator(line: &str) -> Option<(&'static str, &str)> {
    ["~", "+", "-"].into_iter().find_map(|indicator| {
        let rest = line.strip_prefix(indicator)?.strip_prefix(' ')?;
        Some((indicator, rest))
    })
}

/// Detect if a line starts a multi-line array change.
/// Returns (change_type, attr_name) if it does.
/// Examples:
///   ~ roles = [
///   + new_attr = [
///   - old_attr = [
fn detect_multiline_array_start(line: &str) -> Option<(String, String)> {
    let (change_type, clean) = split_change_indicator(line.trim())?;

    if let Some(eq_pos) = clean.find(" = ") {
        let value_part = clean[eq_pos + 3..].trim();
        // Check for array opening that doesn't close on the same line
        if value_part == "[" || (value_part.starts_with('[') && !value_part.ends_with(']')) {
            let attr_name = clean[..eq_pos].trim().to_string();
            return Some((change_type.to_string(), attr_name));
        }
    }
    None
}

/// Process collected multi-line array content and create Change objects.
/// Handles arrays like:
///   ~ roles = [
///       - "removed-element",
///       + "added-element",
///       # (1 unchanged element hidden)
///   ]
fn flush_multiline_array_change(
    attr_name: &str,
    change_type: &str,
    lines: &[String],
    changes: &mut Vec<Change>,
) {
    let mut removed_elements = Vec::new();
    let mut added_elements = Vec::new();
    let mut unchanged_elements = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // Skip structural lines
        if trimmed == "]" || trimmed == "]," || trimmed == "[" {
            continue;
        }

        // Parse comments like "# (1 unchanged element hidden)"
        if trimmed.starts_with('#') {
            let comment = trimmed.trim_start_matches('#').trim().to_string();
            unchanged_elements.push(json!(comment));
            continue;
        }

        // Parse removed elements within the array
        if let Some(value) = trimmed.strip_prefix("- ") {
            let value = value.trim().trim_end_matches(',');
            removed_elements.push(parse_terraform_value(value));
        }
        // Parse added elements within the array
        else if let Some(value) = trimmed.strip_prefix("+ ") {
            let value = value.trim().trim_end_matches(',');
            added_elements.push(parse_terraform_value(value));
        }
        // Plain elements (unchanged, shown for context)
        else if !trimmed.is_empty() {
            let value = trimmed.trim_end_matches(',');
            let parsed = parse_terraform_value(value);
            unchanged_elements.push(parsed);
        }
    }

    match change_type {
        "~" => {
            // Modification: build before and after arrays
            // Before = removed + unchanged, After = added + unchanged
            let mut before_arr: Vec<Value> = Vec::new();
            let mut after_arr: Vec<Value> = Vec::new();

            before_arr.extend(removed_elements);
            before_arr.extend(unchanged_elements.clone());

            after_arr.extend(added_elements);
            after_arr.extend(unchanged_elements);

            if !before_arr.is_empty() || !after_arr.is_empty() {
                changes.push(Change {
                    path: vec![attr_name.to_string()],
                    before: Some(json!(before_arr)),
                    after: Some(json!(after_arr)),
                    sensitive: false,
                    computed: false,
                });
            }
        }
        "+" => {
            // Addition: all elements are new
            let mut all_elements: Vec<Value> = Vec::new();
            all_elements.extend(added_elements);
            all_elements.extend(unchanged_elements);
            // For + prefix, removed_elements shouldn't exist, but include just in case
            all_elements.extend(removed_elements);

            if !all_elements.is_empty() {
                changes.push(Change {
                    path: vec![attr_name.to_string()],
                    before: None,
                    after: Some(json!(all_elements)),
                    sensitive: false,
                    computed: false,
                });
            }
        }
        "-" => {
            // Removal: all elements are being removed
            let mut all_elements: Vec<Value> = Vec::new();
            all_elements.extend(removed_elements);
            all_elements.extend(unchanged_elements);
            all_elements.extend(added_elements);

            if !all_elements.is_empty() {
                changes.push(Change {
                    path: vec![attr_name.to_string()],
                    before: Some(json!(all_elements)),
                    after: None,
                    sensitive: false,
                    computed: false,
                });
            }
        }
        _ => {}
    }
}

fn parse_terraform_value(value_str: &str) -> Value {
    let trimmed = value_str.trim();

    // Remove quotes if present
    let unquoted = if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };

    // Check for special values
    if unquoted == crate::KNOWN_AFTER_APPLY || unquoted == crate::SENSITIVE_VALUE {
        return json!(unquoted);
    }

    // Try to parse as JSON first
    if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
        return parsed;
    }

    // Check for boolean
    if unquoted == "true" {
        return json!(true);
    }
    if unquoted == "false" {
        return json!(false);
    }

    // Check for null
    if unquoted == "null" || unquoted == "(null)" {
        return Value::Null;
    }

    // Check for number
    if let Ok(num) = unquoted.parse::<i64>() {
        return json!(num);
    }
    if let Ok(num) = unquoted.parse::<f64>() {
        return json!(num);
    }

    // Check for array (simple format)
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        // Try to parse as JSON array
        if let Ok(arr) = serde_json::from_str::<Vec<Value>>(trimmed) {
            return json!(arr);
        }
    }

    // Default to string
    json!(unquoted)
}
