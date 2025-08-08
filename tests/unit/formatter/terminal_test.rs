use tfdiff::formatter::terminal::*;
use tfdiff::models::*;
use tfdiff::parser::cleaner::clean_ansi_codes;
use crate::common::test_data::*;

#[cfg(test)]
mod terminal_formatter_tests {
    use super::*;
    
    #[test]
    fn test_format_terminal_output_basic() {
        let plan = sample_terraform_plan();
        let output = format_terminal_output(&plan);
        
        assert!(clean_ansi_codes(&output).contains("TERRAFORM PLAN DIFF"));
        assert!(clean_ansi_codes(&output).contains("✚ 2 to add"));
        assert!(clean_ansi_codes(&output).contains("↻ 1 to change"));
        assert!(clean_ansi_codes(&output).contains("aws_s3_bucket.test"));
        assert!(clean_ansi_codes(&output).contains("aws_instance.web"));
    }
    
    #[test]
    fn test_format_terminal_output_apply_mode() {
        let mut plan = sample_terraform_plan();
        plan.mode = PlanMode::Apply;
        
        let output = format_terminal_output(&plan);
        assert!(clean_ansi_codes(&output).contains("TERRAFORM APPLY DIFF"));
    }
    
    #[test]
    fn test_format_terminal_output_empty_plan() {
        let plan = TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary::default(),
            resources: Vec::new(),
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };
        
        let output = format_terminal_output(&plan);
        assert!(clean_ansi_codes(&output).contains("TERRAFORM PLAN DIFF"));
        // Should not contain summary box when all counts are 0
        assert!(!output.contains("┌────────────────────────────────┐"));
    }
    
    #[test]
    fn test_format_terminal_output_with_destroy() {
        let plan = TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary {
                add: 1,
                change: 0,
                destroy: 2,
                read: 0,
            },
            resources: vec![
                sample_create_resource(),
                sample_destroy_resource(),
            ],
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };
        
        let output = format_terminal_output(&plan);
        assert!(clean_ansi_codes(&output).contains("✚ 1 to add"));
        assert!(clean_ansi_codes(&output).contains("✖ 2 to destroy"));
        assert!(clean_ansi_codes(&output).contains("✚ CREATE"));
        assert!(clean_ansi_codes(&output).contains("✖ DESTROY"));
    }
    
    #[test]
    fn test_format_terminal_output_with_read() {
        let plan = TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary {
                add: 0,
                change: 0,
                destroy: 0,
                read: 3,
            },
            resources: Vec::new(),
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };
        
        let output = format_terminal_output(&plan);
        assert!(clean_ansi_codes(&output).contains("⇐ 3 to read"));
    }
    
    #[test]
    fn test_format_resource_create() {
        let resource = sample_create_resource();
        let formatted = format_resource(&resource);
        
        assert!(clean_ansi_codes(&formatted).contains("✚ CREATE"));
        assert!(clean_ansi_codes(&formatted).contains("aws_s3_bucket.test"));
        assert!(clean_ansi_codes(&formatted).contains("bucket = \"test-bucket\""));
        assert!(clean_ansi_codes(&formatted).contains("force_destroy = false"));
    }
    
    #[test]
    fn test_format_resource_update() {
        let resource = sample_update_resource();
        let formatted = format_resource(&resource);
        
        assert!(clean_ansi_codes(&formatted).contains("↻ UPDATE"));
        assert!(clean_ansi_codes(&formatted).contains("aws_instance.web"));
        assert!(clean_ansi_codes(&formatted).contains("instance_type = \"t3.small\""));
    }
    
    #[test]
    fn test_format_resource_destroy() {
        let resource = sample_destroy_resource();
        let formatted = format_resource(&resource);
        
        assert!(clean_ansi_codes(&formatted).contains("✖ DESTROY"));
        assert!(clean_ansi_codes(&formatted).contains("aws_instance.legacy"));
    }
    
    #[test]
    fn test_format_resource_no_op() {
        let mut resource = sample_create_resource();
        resource.action = ActionType::NoOp;
        
        let formatted = format_resource(&resource);
        assert!(clean_ansi_codes(&formatted).contains("○ NO-OP"));
    }
    
    #[test]
    fn test_format_resource_read() {
        let mut resource = sample_create_resource();
        resource.action = ActionType::Read;
        
        let formatted = format_resource(&resource);
        assert!(clean_ansi_codes(&formatted).contains("⇐ READ"));
    }
    
    #[test]
    fn test_format_summary_line_all_actions() {
        let summary = Summary {
            add: 5,
            change: 3,
            destroy: 2,
            read: 1,
        };
        
        let formatted = format_summary_line(&summary);
        assert!(clean_ansi_codes(&formatted).contains("✚ 5 to add"));
        assert!(clean_ansi_codes(&formatted).contains("↻ 3 to change"));
        assert!(clean_ansi_codes(&formatted).contains("✖ 2 to destroy"));
        assert!(clean_ansi_codes(&formatted).contains("⇐ 1 to read"));
        assert!(clean_ansi_codes(&formatted).contains("│"));
    }
    
    #[test]
    fn test_format_summary_line_partial_actions() {
        let summary = Summary {
            add: 2,
            change: 0,
            destroy: 1,
            read: 0,
        };
        
        let formatted = format_summary_line(&summary);
        assert!(clean_ansi_codes(&formatted).contains("✚ 2 to add"));
        assert!(!clean_ansi_codes(&formatted).contains("to change"));
        assert!(clean_ansi_codes(&formatted).contains("✖ 1 to destroy"));
        assert!(!clean_ansi_codes(&formatted).contains("to read"));
    }
}