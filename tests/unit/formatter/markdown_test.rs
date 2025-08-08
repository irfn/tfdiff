use tfdiff::formatter::markdown::*;
use tfdiff::models::*;
use crate::common::test_data::*;

#[cfg(test)]
mod markdown_formatter_tests {
    use super::*;
    
    #[test]
    fn test_format_markdown_output_basic() {
        let plan = sample_terraform_plan();
        let output = format_markdown_output(&plan);
        
        assert!(output.contains("# Terraform Plan Report"));
        assert!(output.contains("## Summary"));
        assert!(output.contains("- ✅ **2** resources to add"));
        assert!(output.contains("- 🔄 **1** resources to change"));
        assert!(output.contains("- ❌ **0** resources to destroy"));
    }
    
    #[test]
    fn test_format_markdown_output_apply_mode() {
        let mut plan = sample_terraform_plan();
        plan.mode = PlanMode::Apply;
        
        let output = format_markdown_output(&plan);
        assert!(output.contains("# Terraform Apply Report"));
    }
    
    #[test]
    fn test_format_markdown_output_with_resources() {
        let plan = sample_terraform_plan();
        let output = format_markdown_output(&plan);
        
        assert!(output.contains("## Resource Changes"));
        assert!(output.contains("### ✅ CREATE `aws_s3_bucket.test`"));
        assert!(output.contains("### 🔄 UPDATE `aws_instance.web`"));
        assert!(output.contains("```json"));
    }
    
    #[test]
    fn test_format_markdown_output_with_read_resources() {
        let plan = TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary {
                add: 0,
                change: 0,
                destroy: 0,
                read: 2,
            },
            resources: Vec::new(),
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };
        
        let output = format_markdown_output(&plan);
        assert!(output.contains("- 📖 **2** resources to read"));
    }
    
    #[test]
    fn test_format_markdown_output_with_destroy() {
        let plan = TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary {
                add: 0,
                change: 0,
                destroy: 1,
                read: 0,
            },
            resources: vec![sample_destroy_resource()],
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };
        
        let output = format_markdown_output(&plan);
        assert!(output.contains("- ❌ **1** resources to destroy"));
        assert!(output.contains("### ❌ DESTROY `aws_instance.legacy`"));
    }
    
    #[test]
    fn test_format_markdown_output_with_warnings() {
        let mut plan = sample_terraform_plan();
        plan.warnings = vec![
            Warning {
                message: "This is a test warning".to_string(),
                level: WarningLevel::Warning,
            },
            Warning {
                message: "This is an error".to_string(),
                level: WarningLevel::Error,
            },
            Warning {
                message: "This is info".to_string(),
                level: WarningLevel::Info,
            },
        ];
        
        let output = format_markdown_output(&plan);
        assert!(output.contains("## Warnings"));
        assert!(output.contains("- ⚠️ This is a test warning"));
        assert!(output.contains("- 🚨 This is an error"));
        assert!(output.contains("- ℹ️ This is info"));
    }
    
    #[test]
    fn test_format_markdown_output_with_metadata() {
        let plan = sample_terraform_plan();
        let output = format_markdown_output(&plan);
        
        assert!(output.contains("## Metadata"));
        assert!(output.contains("- **Terraform Version:** 1.5.7"));
        assert!(output.contains("- **Generated:** 2024-01-01T12:00:00Z"));
        assert!(output.contains("- **Duration:** 5s"));
    }
    
    #[test]
    fn test_format_markdown_output_empty_plan() {
        let plan = TerraformPlan::default();
        let output = format_markdown_output(&plan);
        
        assert!(output.contains("# Terraform Plan Report"));
        assert!(output.contains("## Summary"));
        assert!(output.contains("- ✅ **0** resources to add"));
        assert!(!output.contains("## Resource Changes"));
        assert!(!output.contains("## Warnings"));
        assert!(!output.contains("## Metadata"));
    }
    
    #[test]
    fn test_format_markdown_output_action_emojis() {
        let plan = TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary::default(),
            resources: vec![
                Resource {
                    id: "test.create".to_string(),
                    name: "create".to_string(),
                    type_name: "test".to_string(),
                    provider: "test".to_string(),
                    action: ActionType::Create,
                    changes: Vec::new(),
                    attributes: std::collections::HashMap::new(),
                    applied: false,
                },
                Resource {
                    id: "test.update".to_string(),
                    name: "update".to_string(),
                    type_name: "test".to_string(),
                    provider: "test".to_string(),
                    action: ActionType::Update,
                    changes: Vec::new(),
                    attributes: std::collections::HashMap::new(),
                    applied: false,
                },
                Resource {
                    id: "test.destroy".to_string(),
                    name: "destroy".to_string(),
                    type_name: "test".to_string(),
                    provider: "test".to_string(),
                    action: ActionType::Destroy,
                    changes: Vec::new(),
                    attributes: std::collections::HashMap::new(),
                    applied: false,
                },
                Resource {
                    id: "test.read".to_string(),
                    name: "read".to_string(),
                    type_name: "test".to_string(),
                    provider: "test".to_string(),
                    action: ActionType::Read,
                    changes: Vec::new(),
                    attributes: std::collections::HashMap::new(),
                    applied: false,
                },
                Resource {
                    id: "test.noop".to_string(),
                    name: "noop".to_string(),
                    type_name: "test".to_string(),
                    provider: "test".to_string(),
                    action: ActionType::NoOp,
                    changes: Vec::new(),
                    attributes: std::collections::HashMap::new(),
                    applied: false,
                },
            ],
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };
        
        let output = format_markdown_output(&plan);
        assert!(output.contains("### ✅ CREATE"));
        assert!(output.contains("### 🔄 UPDATE"));
        assert!(output.contains("### ❌ DESTROY"));
        assert!(output.contains("### 📖 READ"));
        assert!(output.contains("### ⭕ NO-OP"));
    }
    
    #[test]
    fn test_format_markdown_output_footer() {
        let plan = sample_terraform_plan();
        let output = format_markdown_output(&plan);
        
        assert!(output.contains("---"));
        assert!(output.contains("*Generated by tfdiff*"));
    }
}