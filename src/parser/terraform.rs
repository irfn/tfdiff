use crate::{Result, TerraformPlan, PlanMode, Summary};
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_terraform_output(input: &str) -> Result<TerraformPlan> {
    let cleaned = crate::parser::clean_input(input)?;
    let lines: Vec<&str> = cleaned.lines().collect();
    
    let mode = detect_mode(&lines)?;
    let summary = extract_summary(&lines)?;
    let resources = crate::parser::diff::parse_resource_changes(&lines);
    let warnings = Vec::new(); // TODO: Implement warning extraction
    
    Ok(TerraformPlan {
        mode,
        summary,
        resources,
        warnings,
        ..Default::default()
    })
}

pub fn detect_mode(lines: &[&str]) -> Result<PlanMode> {
    for line in lines {
        if line.contains("Plan:") {
            return Ok(PlanMode::Plan);
        }
        if line.contains("Apply complete!") {
            return Ok(PlanMode::Apply);
        }
    }
    
    // Default to Plan if unclear
    Ok(PlanMode::Plan)
}

pub fn extract_summary(lines: &[&str]) -> Result<Summary> {
    for line in lines {
        if line.contains("Plan:") {
            return parse_plan_summary(line);
        }
        if line.contains("Apply complete!") {
            return parse_apply_summary(line);
        }
    }
    
    Ok(Summary::default())
}

lazy_static! {
    static ref PLAN_SUMMARY_REGEX: Regex = Regex::new(r"Plan:\s*(\d+)\s*to add,\s*(\d+)\s*to change,\s*(\d+)\s*to destroy").unwrap();
    static ref APPLY_SUMMARY_REGEX: Regex = Regex::new(r"Apply complete!\s*Resources:\s*(\d+)\s*added,\s*(\d+)\s*changed,\s*(\d+)\s*destroyed").unwrap();
}

fn parse_plan_summary(line: &str) -> Result<Summary> {
    // Parse "Plan: X to add, Y to change, Z to destroy"
    
    if let Some(captures) = PLAN_SUMMARY_REGEX.captures(line) {
        let add = captures[1].parse().unwrap_or(0);
        let change = captures[2].parse().unwrap_or(0);
        let destroy = captures[3].parse().unwrap_or(0);
        
        return Ok(Summary { add, change, destroy, read: 0 });
    }
    
    Ok(Summary::default())
}

fn parse_apply_summary(line: &str) -> Result<Summary> {
    // Parse "Apply complete! Resources: X added, Y changed, Z destroyed"
    
    if let Some(captures) = APPLY_SUMMARY_REGEX.captures(line) {
        let add = captures[1].parse().unwrap_or(0);
        let change = captures[2].parse().unwrap_or(0);
        let destroy = captures[3].parse().unwrap_or(0);
        
        return Ok(Summary { add, change, destroy, read: 0 });
    }
    
    Ok(Summary::default())
}