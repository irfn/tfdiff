use tfdiff::formatter::json::*;
use crate::common::test_data::*;
use serde_json;

#[cfg(test)]
mod json_formatter_tests {
    use super::*;
    
    #[test]
    fn test_format_json_output_basic() {
        let plan = sample_terraform_plan();
        let result = format_json_output(&plan).unwrap();
        
        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        // Verify structure
        assert!(parsed.get("mode").is_some());
        assert!(parsed.get("summary").is_some());
        assert!(parsed.get("resources").is_some());
        assert!(parsed.get("data_sources").is_some());
        assert!(parsed.get("warnings").is_some());
        assert!(parsed.get("metadata").is_some());
    }
    
    #[test]
    fn test_format_json_output_summary() {
        let plan = sample_terraform_plan();
        let result = format_json_output(&plan).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        let summary = &parsed["summary"];
        assert_eq!(summary["add"], 2);
        assert_eq!(summary["change"], 1);
        assert_eq!(summary["destroy"], 0);
        assert_eq!(summary["read"], 0);
    }
    
    #[test]
    fn test_format_json_output_mode() {
        let mut plan = sample_terraform_plan();
        plan.mode = tfdiff::models::PlanMode::Apply;
        
        let result = format_json_output(&plan).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["mode"], "Apply");
    }
    
    #[test]
    fn test_format_json_output_resources() {
        let plan = sample_terraform_plan();
        let result = format_json_output(&plan).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        let resources = parsed["resources"].as_array().unwrap();
        assert_eq!(resources.len(), 2);
        
        let first_resource = &resources[0];
        assert_eq!(first_resource["id"], "aws_s3_bucket.test");
        assert_eq!(first_resource["name"], "test");
        assert_eq!(first_resource["type_name"], "aws_s3_bucket");
        assert_eq!(first_resource["action"], "Create");
    }
    
    #[test]
    fn test_format_json_output_metadata() {
        let plan = sample_terraform_plan();
        let result = format_json_output(&plan).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        let metadata = &parsed["metadata"];
        assert_eq!(metadata["terraform_version"], "1.5.7");
        assert_eq!(metadata["timestamp"], "2024-01-01T12:00:00Z");
        assert_eq!(metadata["duration"], "5s");
    }
    
    #[test]
    fn test_format_json_output_empty_plan() {
        let plan = tfdiff::models::TerraformPlan::default();
        let result = format_json_output(&plan).unwrap();
        
        // Should be valid JSON even with empty plan
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["summary"]["add"], 0);
        assert_eq!(parsed["summary"]["change"], 0);
        assert_eq!(parsed["summary"]["destroy"], 0);
        assert_eq!(parsed["resources"].as_array().unwrap().len(), 0);
    }
    
    #[test]
    fn test_format_json_output_pretty_printed() {
        let plan = sample_terraform_plan();
        let result = format_json_output(&plan).unwrap();
        
        // Should be pretty-printed (contains indentation)
        assert!(result.contains("  \"mode\""));
        assert!(result.contains("    \"add\""));
        
        // Should be parseable
        let _parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    }
    
    #[test]
    fn test_json_roundtrip() {
        let original_plan = sample_terraform_plan();
        let json_output = format_json_output(&original_plan).unwrap();
        let parsed_plan: tfdiff::models::TerraformPlan = serde_json::from_str(&json_output).unwrap();
        
        // Verify roundtrip consistency
        assert_eq!(original_plan.summary.add, parsed_plan.summary.add);
        assert_eq!(original_plan.summary.change, parsed_plan.summary.change);
        assert_eq!(original_plan.resources.len(), parsed_plan.resources.len());
        assert_eq!(original_plan.resources[0].id, parsed_plan.resources[0].id);
    }
}