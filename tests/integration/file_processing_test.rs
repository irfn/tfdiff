use tfdiff::parser::*;
use tfdiff::formatter::*;
use tfdiff::models::PlanMode;
use tfdiff::parser::cleaner::clean_ansi_codes;
use crate::common::fixtures::*;
use crate::common::assertions::*;

#[cfg(test)]
mod file_processing_integration_tests {
    use super::*;
    
    #[test]
    fn test_process_simple_plan_fixture() {
        let content = load_terraform_fixture("simple_plan.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 1, 0, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_complex_update_fixture() {
        let content = load_terraform_fixture("complex_update.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 1, 1, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_apply_output_fixture() {
        let content = load_terraform_fixture("apply_output.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 3, 0, 0);
        assert!(matches!(result.mode, PlanMode::Apply));
    }
    
    #[test]
    fn test_process_destroy_plan_fixture() {
        let content = load_terraform_fixture("destroy_plan.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 0, 0, 2);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_module_plan_fixture() {
        let content = load_terraform_fixture("module_plan.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 4, 0, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_ansi_codes_fixture() {
        let content = load_edge_case_fixture("ansi_codes.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 1, 0, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_cdk_prefix_fixture() {
        let content = load_edge_case_fixture("cdk_prefix.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 2, 0, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_large_plan_fixture() {
        let content = load_edge_case_fixture("large_plan.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 10, 0, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_aws_s3_lifecycle_fixture() {
        let content = load_provider_fixture("aws", "s3_lifecycle.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 1, 1, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_kubernetes_deployment_fixture() {
        let content = load_provider_fixture("kubernetes", "deployment.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        assert_summary_totals(&result.summary, 1, 1, 0);
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_process_incomplete_plan_fixture() {
        let content = load_malformed_fixture("incomplete_plan.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        // Should still parse what it can
        assert!(matches!(result.mode, PlanMode::Plan));
        // Summary should be default since no complete plan line
        assert_summary_totals(&result.summary, 0, 0, 0);
    }
    
    #[test]
    fn test_process_corrupted_json_fixture() {
        let content = load_malformed_fixture("corrupted_json.txt");
        let result = parse_terraform_output(&content).unwrap();
        
        // Should handle malformed JSON gracefully
        assert!(matches!(result.mode, PlanMode::Plan));
    }
    
    #[test]
    fn test_end_to_end_terminal_formatting() {
        let content = load_terraform_fixture("simple_plan.txt");
        let parsed = parse_terraform_output(&content).unwrap();
        let formatted = format_terminal_output(&parsed);
        
        assert!(clean_ansi_codes(&formatted).contains("TERRAFORM PLAN ANALYSIS"));
        assert!(clean_ansi_codes(&formatted).contains("1 resources to add"));
        assert!(!clean_ansi_codes(&formatted).contains("to change"));
        assert!(!clean_ansi_codes(&formatted).contains("to destroy"));
    }
    
    #[test]
    fn test_end_to_end_json_formatting() {
        let content = load_terraform_fixture("complex_update.txt");
        let parsed = parse_terraform_output(&content).unwrap();
        let formatted = format_json_output(&parsed).unwrap();
        
        // Should be valid JSON
        let json_value: serde_json::Value = serde_json::from_str(&formatted).unwrap();
        assert_eq!(json_value["summary"]["add"], 1);
        assert_eq!(json_value["summary"]["change"], 1);
        assert_eq!(json_value["summary"]["destroy"], 0);
    }
    
    #[test]
    fn test_end_to_end_html_formatting() {
        let content = load_terraform_fixture("destroy_plan.txt");
        let parsed = parse_terraform_output(&content).unwrap();
        let formatted = format_html_output(&parsed);
        
        assert!(formatted.contains("<!DOCTYPE html>"));
        assert!(formatted.contains("Terraform Plan Analysis"));
        assert!(formatted.contains(">2<"));
        assert!(formatted.contains("to destroy"));
        assert!(formatted.contains("</html>"));
    }
    
    #[test]
    fn test_end_to_end_markdown_formatting() {
        let content = load_terraform_fixture("module_plan.txt");
        let parsed = parse_terraform_output(&content).unwrap();
        let formatted = format_markdown_output(&parsed);
        
        assert!(formatted.contains("# Terraform Plan Report"));
        assert!(formatted.contains("## Summary"));
        assert!(formatted.contains("- ✅ **4** resources to add"));
        assert!(formatted.contains("*Generated by tfdiff*"));
    }
    
    #[test]
    fn test_performance_large_fixture() {
        let content = load_edge_case_fixture("large_plan.txt");
        
        let start = std::time::Instant::now();
        let parsed = parse_terraform_output(&content).unwrap();
        let parse_duration = start.elapsed();
        
        let format_start = std::time::Instant::now();
        let _formatted = format_terminal_output(&parsed);
        let format_duration = format_start.elapsed();
        
        // Performance assertions - should be fast
        assert!(parse_duration.as_millis() < 100, "Parsing took too long: {:?}", parse_duration);
        assert!(format_duration.as_millis() < 50, "Formatting took too long: {:?}", format_duration);
        
        // Correctness assertions
        assert_summary_totals(&parsed.summary, 10, 0, 0);
    }
    
    #[test]
    fn test_error_handling_empty_input() {
        let result = parse_terraform_output("");
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_summary_totals(&parsed.summary, 0, 0, 0);
    }
    
    #[test]
    fn test_error_handling_invalid_input() {
        let invalid_input = "This is not terraform output at all!";
        let result = parse_terraform_output(invalid_input);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_summary_totals(&parsed.summary, 0, 0, 0);
    }
}