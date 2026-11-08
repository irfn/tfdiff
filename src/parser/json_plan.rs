use crate::{
    ActionType, Change, Metadata, PlanMode, Resource, Result, Summary, TerraformPlan, TfdiffError,
    Warning, WarningLevel, KNOWN_AFTER_APPLY, SENSITIVE_VALUE,
};
use serde::de::IgnoredAny;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::{BTreeSet, HashMap};

// Subset of the `terraform show -json <planfile>` representation that tfdiff needs.
// See https://developer.hashicorp.com/terraform/internals/json-format
#[derive(Deserialize)]
struct JsonPlan {
    format_version: Option<String>,
    terraform_version: Option<String>,
    timestamp: Option<String>,
    // Always present in plan output; distinguishes a plan from `terraform show -json` of state
    planned_values: Option<IgnoredAny>,
    #[serde(default)]
    errored: bool,
    #[serde(default)]
    resource_changes: Vec<JsonResourceChange>,
}

#[derive(Deserialize)]
struct JsonResourceChange {
    address: String,
    #[serde(rename = "type")]
    type_name: String,
    name: String,
    provider_name: String,
    deposed: Option<String>,
    change: JsonChange,
}

#[derive(Deserialize)]
struct JsonChange {
    actions: Vec<String>,
    #[serde(default)]
    before: Value,
    #[serde(default)]
    after: Value,
    #[serde(default)]
    after_unknown: Value,
    #[serde(default)]
    before_sensitive: Value,
    #[serde(default)]
    after_sensitive: Value,
    importing: Option<IgnoredAny>,
}

/// Returns true if the input looks like `terraform show -json` output rather than Terraform's
/// human-readable output. The JSON lines of `terraform plan -json` are not a plan document;
/// they are left to the text parser, which picks up the summary from their messages.
pub fn is_json_input(input: &str) -> bool {
    let input = input.trim_start();
    input.starts_with('{') && !input.starts_with(r#"{"@level""#)
}

/// Parse the output of `terraform show -json <planfile>`.
///
/// Sensitive values are always replaced with "(sensitive value)": the JSON contains them in
/// plain text and only marks their location in `before_sensitive` / `after_sensitive`.
pub fn parse_terraform_json(input: &str) -> Result<TerraformPlan> {
    let plan: JsonPlan = serde_json::from_str(input).map_err(|e| {
        TfdiffError::ParseError(format!(
            "input is JSON but not valid `terraform show -json` plan output: {}",
            e
        ))
    })?;

    if plan.format_version.is_none() || plan.planned_values.is_none() {
        return Err(TfdiffError::ParseError(
            "input is JSON but not a plan; expected `terraform show -json <planfile>` output"
                .to_string(),
        ));
    }

    let mut warnings = Vec::new();
    if plan.errored {
        warnings.push(Warning {
            message: "Terraform reported errors while planning; this plan is incomplete"
                .to_string(),
            level: WarningLevel::Error,
        });
    }

    let mut summary = Summary::default();
    let mut resources = Vec::new();

    let mut unsupported = BTreeSet::new();

    for rc in plan.resource_changes {
        let Some(action) = action_type(&rc.change.actions) else {
            if rc.change.importing.is_some() {
                unsupported.insert("import".to_string());
            } else if rc.change.actions != ["no-op"] {
                unsupported.insert(rc.change.actions.join("/"));
            }
            continue;
        };

        match action {
            ActionType::Create => summary.add += 1,
            ActionType::Update => summary.change += 1,
            ActionType::Destroy => summary.destroy += 1,
            ActionType::Replace => {
                summary.add += 1;
                summary.destroy += 1;
            }
            ActionType::Read => summary.read += 1,
            ActionType::NoOp => {}
        }

        resources.push(build_resource(rc, action));
    }

    if !unsupported.is_empty() {
        warnings.push(Warning {
            message: format!(
                "Some resource changes are not shown because tfdiff does not support these actions yet: {}",
                Vec::from_iter(unsupported).join(", ")
            ),
            level: WarningLevel::Warning,
        });
    }

    Ok(TerraformPlan {
        mode: PlanMode::Plan,
        summary,
        resources,
        warnings,
        metadata: Metadata {
            terraform_version: plan.terraform_version,
            timestamp: plan.timestamp,
            duration: None,
        },
        ..Default::default()
    })
}

fn action_type(actions: &[String]) -> Option<ActionType> {
    let actions: Vec<&str> = actions.iter().map(String::as_str).collect();
    match actions.as_slice() {
        ["create"] => Some(ActionType::Create),
        ["update"] => Some(ActionType::Update),
        ["delete"] => Some(ActionType::Destroy),
        ["read"] => Some(ActionType::Read),
        ["delete", "create"] | ["create", "delete"] => Some(ActionType::Replace),
        // "no-op" is not listed in Terraform's text output either; "forget" and other
        // actions have no equivalent in the model yet and are reported as a warning.
        _ => None,
    }
}

fn build_resource(rc: JsonResourceChange, action: ActionType) -> Resource {
    let id = match rc.deposed {
        Some(deposed) => format!("{} (deposed object {})", rc.address, deposed),
        None => rc.address,
    };
    let change = rc.change;
    let changes = match action {
        ActionType::Update | ActionType::Replace => attribute_changes(&change),
        _ => Vec::new(),
    };
    let attributes = match action {
        ActionType::Create | ActionType::Read => collect_attributes(resolve(
            change.after,
            &change.after_unknown,
            &change.after_sensitive,
        )),
        ActionType::Destroy => collect_attributes(resolve(
            change.before,
            &Value::Null,
            &change.before_sensitive,
        )),
        _ => HashMap::new(),
    };

    Resource {
        id,
        name: rc.name,
        type_name: rc.type_name,
        provider: rc.provider_name,
        action,
        changes,
        attributes,
        applied: false,
    }
}

fn collect_attributes(value: Value) -> HashMap<String, Value> {
    match value {
        // Terraform's text output omits null attributes as well
        Value::Object(map) => map.into_iter().filter(|(_, v)| !v.is_null()).collect(),
        _ => HashMap::new(),
    }
}

/// One `Change` per top-level attribute whose value differs or becomes unknown.
fn attribute_changes(change: &JsonChange) -> Vec<Change> {
    let keys: BTreeSet<&String> = [&change.before, &change.after, &change.after_unknown]
        .into_iter()
        .filter_map(Value::as_object)
        .flat_map(Map::keys)
        .collect();

    keys.into_iter()
        .filter_map(|key| {
            let key = key.as_str();
            let (before, after, unknown) = (
                &change.before[key],
                &change.after[key],
                child(&change.after_unknown, key),
            );
            let computed = contains_true(unknown);
            // Compare the raw values so changes to sensitive values are still detected
            if before == after && !computed {
                return None;
            }

            // Like Terraform, mask both sides when either side is sensitive, so a value that
            // just became sensitive doesn't show its old plain text value
            let sensitive = &merge_sensitive(
                child(&change.before_sensitive, key),
                child(&change.after_sensitive, key),
            );
            Some(Change {
                path: vec![key.to_string()],
                before: non_null(resolve(before.clone(), &Value::Null, sensitive)),
                after: non_null(resolve(after.clone(), unknown, sensitive)),
                sensitive: contains_true(sensitive),
                computed,
            })
        })
        .collect()
}

/// Merge unknown markers into `value` and mask sensitive parts.
fn resolve(value: Value, unknown: &Value, sensitive: &Value) -> Value {
    if sensitive == &Value::Bool(true) {
        return json!(SENSITIVE_VALUE);
    }
    if unknown == &Value::Bool(true) {
        return json!(KNOWN_AFTER_APPLY);
    }

    match value {
        Value::Object(map) => {
            let mut resolved: Map<String, Value> = map
                .into_iter()
                .map(|(k, v)| {
                    let v = resolve(v, &unknown[k.as_str()], &sensitive[k.as_str()]);
                    (k, v)
                })
                .collect();
            // Attributes that are entirely unknown are absent from the value itself
            for (k, u) in unknown.as_object().into_iter().flatten() {
                resolved
                    .entry(k.clone())
                    .or_insert_with(|| resolve(Value::Null, u, &sensitive[k.as_str()]));
            }
            Value::Object(resolved)
        }
        Value::Null if unknown.is_object() => {
            resolve(Value::Object(Map::new()), unknown, sensitive)
        }
        Value::Array(items) => items
            .into_iter()
            .enumerate()
            .map(|(i, v)| resolve(v, &unknown[i], &sensitive[i]))
            .collect(),
        other => other,
    }
}

/// Union of two sensitivity markers: a path is sensitive if it is in either.
fn merge_sensitive(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Bool(true), _) | (_, Value::Bool(true)) => Value::Bool(true),
        (Value::Object(x), Value::Object(y)) => Value::Object(
            x.keys()
                .chain(y.keys().filter(|k| !x.contains_key(*k)))
                .map(|k| (k.clone(), merge_sensitive(&a[k.as_str()], &b[k.as_str()])))
                .collect(),
        ),
        (Value::Array(x), Value::Array(y)) => (0..x.len().max(y.len()))
            .map(|i| merge_sensitive(&a[i], &b[i]))
            .collect(),
        // A value that changes shape: mask all of it rather than guess which parts match
        (Value::Object(_) | Value::Array(_), Value::Object(_) | Value::Array(_)) => {
            Value::Bool(true)
        }
        (Value::Object(_) | Value::Array(_), _) => a.clone(),
        _ => b.clone(),
    }
}

/// The marker for `key`; a marker that is `true` applies to everything below it.
fn child<'a>(marker: &'a Value, key: &str) -> &'a Value {
    static TRUE: Value = Value::Bool(true);
    if marker == &TRUE {
        &TRUE
    } else {
        &marker[key]
    }
}

fn non_null(value: Value) -> Option<Value> {
    (!value.is_null()).then_some(value)
}

fn contains_true(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::Object(map) => map.values().any(contains_true),
        Value::Array(items) => items.iter().any(contains_true),
        _ => false,
    }
}
