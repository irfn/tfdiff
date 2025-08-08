use tfdiff::parser::*;
use tfdiff::models::*;

#[cfg(test)]
mod terraform_parser_tests {
    use super::*;
    
    #[test]
    fn test_detect_plan_mode() {
        let lines = vec![
            "Terraform used the selected providers to generate the following execution plan.",
            "Plan: 2 to add, 1 to change, 0 to destroy."
        ];
        
        let result = detect_mode(&lines).unwrap();
        assert!(matches!(result, PlanMode::Plan));
    }
    
    #[test]
    fn test_detect_apply_mode() {
        let lines = vec![
            "aws_s3_bucket.example: Creating...",
            "aws_s3_bucket.example: Creation complete after 2s",
            "Apply complete! Resources: 1 added, 0 changed, 0 destroyed."
        ];
        
        let result = detect_mode(&lines).unwrap();
        assert!(matches!(result, PlanMode::Apply));
    }
    
    #[test]
    fn test_detect_mode_defaults_to_plan() {
        let lines = vec![
            "Some random terraform output",
            "Without clear indicators"
        ];
        
        let result = detect_mode(&lines).unwrap();
        assert!(matches!(result, PlanMode::Plan));
    }
    
    #[test]
    fn test_extract_plan_summary_basic() {
        let lines = vec![
            "Terraform will perform the following actions:",
            "Plan: 2 to add, 1 to change, 0 to destroy."
        ];
        
        let result = extract_summary(&lines).unwrap();
        assert_eq!(result.add, 2);
        assert_eq!(result.change, 1);
        assert_eq!(result.destroy, 0);
        assert_eq!(result.read, 0);
    }
    
    #[test]
    fn test_extract_plan_summary_with_destroy() {
        let lines = vec![
            "Plan: 0 to add, 0 to change, 5 to destroy."
        ];
        
        let result = extract_summary(&lines).unwrap();
        assert_eq!(result.add, 0);
        assert_eq!(result.change, 0);
        assert_eq!(result.destroy, 5);
    }
    
    #[test]
    fn test_extract_apply_summary() {
        let lines = vec![
            "aws_instance.web: Creating...",
            "Apply complete! Resources: 3 added, 2 changed, 1 destroyed."
        ];
        
        let result = extract_summary(&lines).unwrap();
        assert_eq!(result.add, 3);
        assert_eq!(result.change, 2);
        assert_eq!(result.destroy, 1);
    }
    
    #[test]
    fn test_extract_summary_no_match_returns_default() {
        let lines = vec![
            "Some terraform output",
            "Without summary information"
        ];
        
        let result = extract_summary(&lines).unwrap();
        assert_eq!(result.add, 0);
        assert_eq!(result.change, 0);
        assert_eq!(result.destroy, 0);
        assert_eq!(result.read, 0);
    }
    
    #[test]
    fn test_parse_terraform_output_basic() {
        let input = r#"
Terraform used the selected providers to generate the following execution plan.

Plan: 1 to add, 0 to change, 0 to destroy.
        "#;
        
        let result = parse_terraform_output(input).unwrap();
        assert!(matches!(result.mode, PlanMode::Plan));
        assert_eq!(result.summary.add, 1);
        assert_eq!(result.summary.change, 0);
        assert_eq!(result.summary.destroy, 0);
    }
    
    #[test]
    fn test_parse_terraform_output_with_ansi_codes() {
        let input = "\x1b[32mPlan: 2 to add, 1 to change, 0 to destroy.\x1b[0m";
        
        let result = parse_terraform_output(input).unwrap();
        assert_eq!(result.summary.add, 2);
        assert_eq!(result.summary.change, 1);
        assert_eq!(result.summary.destroy, 0);
    }
    
    #[test]
    fn test_parse_terraform_output_apply() {
        let input = r#"
aws_s3_bucket.test: Creating...
aws_s3_bucket.test: Creation complete after 1s

Apply complete! Resources: 1 added, 0 changed, 0 destroyed.
        "#;
        
        let result = parse_terraform_output(input).unwrap();
        assert!(matches!(result.mode, PlanMode::Apply));
        assert_eq!(result.summary.add, 1);
        assert_eq!(result.summary.change, 0);
        assert_eq!(result.summary.destroy, 0);
    }
}