use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Placeholder for a value Terraform only knows after applying the plan.
pub const KNOWN_AFTER_APPLY: &str = "(known after apply)";
/// Placeholder that replaces a sensitive value in all output.
pub const SENSITIVE_VALUE: &str = "(sensitive value)";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformPlan {
    pub mode: PlanMode,
    pub summary: Summary,
    pub resources: Vec<Resource>,
    pub data_sources: Vec<DataSource>,
    pub warnings: Vec<Warning>,
    pub metadata: Metadata,
}

impl Default for TerraformPlan {
    fn default() -> Self {
        Self {
            mode: PlanMode::Plan,
            summary: Summary::default(),
            resources: Vec::new(),
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlanMode {
    Plan,
    Apply,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Summary {
    pub add: usize,
    pub change: usize,
    pub destroy: usize,
    pub read: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub type_name: String,
    pub provider: String,
    pub action: ActionType,
    pub changes: Vec<Change>,
    pub attributes: HashMap<String, serde_json::Value>,
    pub applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub id: String,
    pub name: String,
    pub type_name: String,
    pub provider: String,
    pub attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Create,
    Update,
    Destroy,
    Replace,
    Read,
    NoOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub path: Vec<String>,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
    pub sensitive: bool,
    pub computed: bool,
}

impl Change {
    /// True if the value differs between before and after. Sensitive values are masked to
    /// the same placeholder on both sides, so a sensitive change always counts as modified.
    pub fn is_modified(&self) -> bool {
        self.before != self.after || self.sensitive
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub message: String,
    pub level: WarningLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WarningLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub terraform_version: Option<String>,
    pub timestamp: Option<String>,
    pub duration: Option<String>,
}
