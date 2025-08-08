use crate::{TerraformPlan, Result};

pub fn format_json_output(plan: &TerraformPlan) -> Result<String> {
    let json = serde_json::to_string_pretty(plan)?;
    Ok(json)
}