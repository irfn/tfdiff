use crate::{ActionType, Change, TerraformPlan, KNOWN_AFTER_APPLY, SENSITIVE_VALUE};
use colored::*;
use serde_json::{Map, Value};
use std::collections::{BTreeSet, HashMap};

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
    if plan.summary.add > 0
        || plan.summary.change > 0
        || plan.summary.destroy > 0
        || plan.summary.read > 0
    {
        output.push_str("📊 ");
        output.push_str(&"PLAN SUMMARY".bright_white().bold().to_string());
        output.push('\n');
        output.push_str(&"─".repeat(80).dimmed().to_string());
        output.push('\n');
        output.push('\n');

        // Summary stats in a grid layout
        if plan.summary.add > 0 {
            output.push_str(&format!(
                "  {} {} resources to add\n",
                "➕".bright_green(),
                plan.summary.add.to_string().bright_green().bold()
            ));
        }
        if plan.summary.change > 0 {
            output.push_str(&format!(
                "  {} {} resources to change\n",
                "🔄".bright_yellow(),
                plan.summary.change.to_string().bright_yellow().bold()
            ));
        }
        if plan.summary.destroy > 0 {
            output.push_str(&format!(
                "  {} {} resources to destroy\n",
                "🗑️".bright_red(),
                plan.summary.destroy.to_string().bright_red().bold()
            ));
        }
        if plan.summary.read > 0 {
            output.push_str(&format!(
                "  {} {} data sources to read\n",
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
        output.push_str("🔧 ");
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
        output.push_str("📊 ");
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
        output.push_str("⚠️  ");
        output.push_str(&"WARNINGS".bright_yellow().bold().to_string());
        output.push('\n');
        output.push_str(&"─".repeat(80).dimmed().to_string());
        output.push('\n');
        output.push('\n');

        for warning in &plan.warnings {
            output.push_str(&format!(
                "  {} {}\n",
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
        parts.push(
            format!("✚ {} to add", summary.add)
                .bright_green()
                .to_string(),
        );
    }
    if summary.change > 0 {
        parts.push(
            format!("↻ {} to change", summary.change)
                .bright_yellow()
                .to_string(),
        );
    }
    if summary.destroy > 0 {
        parts.push(
            format!("✖ {} to destroy", summary.destroy)
                .bright_red()
                .to_string(),
        );
    }
    if summary.read > 0 {
        parts.push(
            format!("⇐ {} to read", summary.read)
                .bright_cyan()
                .to_string(),
        );
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
        ActionType::Replace => format!(" {} ", "REPLACE")
            .on_bright_magenta()
            .black()
            .bold(),
        ActionType::Read => format!(" {} ", "READ").on_bright_cyan().black().bold(),
        ActionType::NoOp => format!(" {} ", "NO-OP").on_bright_black().white().bold(),
    };

    output.push_str(&format!(
        "{} {}\n",
        action_badge,
        resource.id.bright_white().bold()
    ));

    // Show resource type and provider if available
    if !resource.type_name.is_empty() {
        output.push_str(&format!(
            "  {} {}\n",
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
                            output.push_str(&format!(
                                "  {} {} = {}\n",
                                "-".bright_red().bold(),
                                key.bright_cyan(),
                                before.bright_red()
                            ));
                            continue;
                        }

                        output.push_str(&format_modification(key, before, after));
                        continue;
                    }
                }
            }

            let formatted_value = format_json_value(value, 1);
            output.push_str(&format!("  {} = {}\n", key.bright_cyan(), formatted_value));
        }
    }

    output.push_str(&"─".repeat(80).dimmed().to_string());
    output.push('\n');

    output
}

fn format_changes(changes: &[Change], _action: &ActionType) -> String {
    let mut output = String::new();

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
                output.push_str(&format!(
                    "  {} {} = {}\n",
                    "+".bright_green().bold(),
                    path_str.bright_cyan(),
                    format_json_value(after, 2).bright_green()
                ));
            }
            (Some(before), None) => {
                // Deletion
                output.push_str(&format!(
                    "  {} {} = {}\n",
                    "-".bright_red().bold(),
                    path_str.bright_cyan(),
                    format_json_value(before, 2).bright_red()
                ));
            }
            (Some(before), Some(after)) if change.is_modified() => {
                // Check if this is a DESTROY action with null after value
                if let ActionType::Destroy = _action {
                    if after == &Value::Null {
                        // For DESTROY with null, just show the value being removed
                        output.push_str(&format!(
                            "  {} {} = {}\n",
                            "-".bright_red().bold(),
                            path_str.bright_cyan(),
                            format_json_value(before, 2).bright_red()
                        ));
                        continue;
                    }
                }

                if is_collection(before) || is_collection(after) {
                    output.push_str(&format_collection_modification(&path_str, before, after));
                } else {
                    output.push_str(&format_modification(
                        &path_str,
                        &plain_value(before),
                        &plain_value(after),
                    ));
                }
            }
            _ => {
                // No change or computed values
                if change.computed {
                    output.push_str(&format!(
                        "  {} {} = {}\n",
                        "?".bright_blue().bold(),
                        path_str.bright_cyan(),
                        "(known after apply)".italic().dimmed()
                    ));
                }
            }
        }

        if change.sensitive {
            output.push_str(&format!(
                "    {} {}\n",
                "🔒",
                "(sensitive value)".italic().dimmed()
            ));
        }
    }

    output
}

/// A modified attribute: its name, then the old and new value side by side in aligned columns.
fn format_modification(name: &str, before: &str, after: &str) -> String {
    format!(
        "  {} {}\n    {} {:<40} │ {} {}\n",
        "~".bright_yellow().bold(),
        name.bright_cyan(),
        "─".bright_red(),
        truncate(before, 40).bright_red(),
        "+".bright_green(),
        truncate(after, 40).bright_green()
    )
}

fn is_collection(value: &Value) -> bool {
    value.is_array() || value.is_object()
}

/// A modified list or map doesn't fit in the side-by-side columns, so list only what changed
/// inside it, like Terraform does.
fn format_collection_modification(name: &str, before: &Value, after: &Value) -> String {
    let mut output = format!("  {} {}\n", "~".bright_yellow().bold(), name.bright_cyan());
    push_value_diff(&mut output, before, after, 2);
    output
}

fn push_value_diff(output: &mut String, before: &Value, after: &Value, depth: usize) {
    match (before, after) {
        (Value::Array(before), Value::Array(after)) => {
            push_array_diff(output, before, after, depth)
        }
        (Value::Object(before), Value::Object(after)) => {
            push_object_diff(output, before, after, depth)
        }
        _ => {
            push_value_lines(output, Sign::Remove, None, before, depth);
            push_value_lines(output, Sign::Add, None, after, depth);
        }
    }
}

/// Compares elements by value rather than position: most long lists in plans are sets (CIDRs,
/// IDs, ...), where only the added and removed elements are of interest.
fn push_array_diff(output: &mut String, before: &[Value], after: &[Value], depth: usize) {
    // The text parser keeps Terraform's "(N unchanged elements hidden)" lines as elements
    let hidden = |items: &[Value]| items.iter().filter_map(hidden_element_count).sum::<usize>();
    let hidden = hidden(before).max(hidden(after));
    let shown = |items: &[Value]| -> Vec<Value> {
        items
            .iter()
            .filter(|v| hidden_element_count(v).is_none())
            .cloned()
            .collect()
    };
    let (before, after) = (shown(before), shown(after));

    // Elements of `items` that have no counterpart in `other`, matching duplicates one-to-one
    fn unmatched<'a>(items: &'a [Value], other: &[Value]) -> Vec<&'a Value> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for item in other {
            *counts.entry(item.to_string()).or_default() += 1;
        }
        items
            .iter()
            .filter(|item| match counts.get_mut(&item.to_string()) {
                Some(count) if *count > 0 => {
                    *count -= 1;
                    false
                }
                _ => true,
            })
            .collect()
    }
    let removed = unmatched(&before, &after);
    let added = unmatched(&after, &before);
    let unchanged = before.len() - removed.len() + hidden;

    for item in removed {
        push_value_lines(output, Sign::Remove, None, item, depth);
    }
    for item in added {
        push_value_lines(output, Sign::Add, None, item, depth);
    }
    if unchanged > 0 {
        let noun = if unchanged == 1 {
            "element"
        } else {
            "elements"
        };
        push_hidden_line(output, unchanged, noun, depth);
    }
}

fn push_object_diff(
    output: &mut String,
    before: &Map<String, Value>,
    after: &Map<String, Value>,
    depth: usize,
) {
    let keys: BTreeSet<&String> = before.keys().chain(after.keys()).collect();
    let mut unchanged = 0;
    for key in keys {
        match (before.get(key), after.get(key)) {
            (Some(b), Some(a)) if b == a => unchanged += 1,
            (Some(b), Some(a)) if is_collection(b) || is_collection(a) => {
                output.push_str(&format!(
                    "{}{} {}\n",
                    "  ".repeat(depth),
                    "~".bright_yellow().bold(),
                    key.bright_cyan()
                ));
                push_value_diff(output, b, a, depth + 1);
            }
            (Some(b), Some(a)) => output.push_str(&format!(
                "{}{} {} = {} {} {}\n",
                "  ".repeat(depth),
                "~".bright_yellow().bold(),
                key.bright_cyan(),
                plain_value(b).bright_red(),
                "→".dimmed(),
                plain_value(a).bright_green()
            )),
            (Some(b), None) => push_value_lines(output, Sign::Remove, Some(key), b, depth),
            (None, Some(a)) => push_value_lines(output, Sign::Add, Some(key), a, depth),
            (None, None) => {}
        }
    }
    if unchanged > 0 {
        let noun = if unchanged == 1 {
            "attribute"
        } else {
            "attributes"
        };
        push_hidden_line(output, unchanged, noun, depth);
    }
}

#[derive(Clone, Copy)]
enum Sign {
    Add,
    Remove,
}

/// One added or removed value; collections are pretty-printed with every line colored on its
/// own, so that a color never spans a line break.
fn push_value_lines(
    output: &mut String,
    sign: Sign,
    key: Option<&str>,
    value: &Value,
    depth: usize,
) {
    let text = match value {
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
        }
        _ => plain_value(value),
    };
    let text = match key {
        Some(key) => format!("{} = {}", key, text),
        None => text,
    };
    let indent = "  ".repeat(depth);
    for (i, line) in text.lines().enumerate() {
        let marker = if i == 0 {
            match sign {
                Sign::Add => "+".bright_green().bold(),
                Sign::Remove => "-".bright_red().bold(),
            }
        } else {
            " ".normal()
        };
        let line = match sign {
            Sign::Add => line.bright_green(),
            Sign::Remove => line.bright_red(),
        };
        output.push_str(&format!("{}{} {}\n", indent, marker, line));
    }
}

fn push_hidden_line(output: &mut String, count: usize, noun: &str, depth: usize) {
    output.push_str(&format!(
        "{}  {}\n",
        "  ".repeat(depth),
        format!("# ({} unchanged {} hidden)", count, noun).dimmed()
    ));
}

/// The count from a "(N unchanged elements hidden)" placeholder element.
fn hidden_element_count(value: &Value) -> Option<usize> {
    let (count, rest) = value
        .as_str()?
        .strip_prefix('(')?
        .strip_suffix(')')?
        .split_once(' ')?;
    matches!(
        rest,
        "unchanged element hidden" | "unchanged elements hidden"
    )
    .then(|| count.parse().ok())
    .flatten()
}

/// A scalar without color codes, so it can be safely truncated and colored by the caller.
fn plain_value(value: &Value) -> String {
    match value {
        Value::String(s) if s == KNOWN_AFTER_APPLY || s == SENSITIVE_VALUE => s.clone(),
        Value::String(s) => format!("\"{}\"", s),
        other => other.to_string(),
    }
}

/// Shorten to at most `max_width` characters, cutting on a char boundary.
fn truncate(s: &str, max_width: usize) -> String {
    if s.chars().count() > max_width {
        format!("{}...", s.chars().take(max_width - 3).collect::<String>())
    } else {
        s.to_string()
    }
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
                    result.push_str(&format!(
                        "{}  {}",
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
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let mut result = "{\n".to_string();
                let entries: Vec<_> = obj.iter().collect();
                for (i, (key, val)) in entries.iter().enumerate() {
                    result.push_str(&format!(
                        "{}  {} = {}",
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
    output.push_str(&format!(
        "{} {}\n",
        action_badge,
        data_source.id.bright_white().bold()
    ));

    if !data_source.type_name.is_empty() {
        output.push_str(&format!(
            "  {} {}\n",
            "Type:".dimmed(),
            data_source.type_name.bright_blue()
        ));
    }

    output.push('\n');
    for (key, value) in &data_source.attributes {
        let formatted_value = format_json_value(value, 1);
        output.push_str(&format!("  {} = {}\n", key.bright_cyan(), formatted_value));
    }

    output.push_str(&"─".repeat(80).dimmed().to_string());
    output.push('\n');

    output
}
