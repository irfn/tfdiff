use tfdiff::formatter::html::*;
use tfdiff::models::*;
use crate::common::test_data::*;

#[cfg(test)]
mod html_formatter_tests {
    use super::*;
    
    #[test]
    fn test_format_html_output_basic() {
        let plan = sample_terraform_plan();
        let output = format_html_output(&plan);
        
        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("<html lang=\"en\">"));
        assert!(output.contains("Terraform Plan Report"));
        assert!(output.contains("📊 Summary"));
        assert!(output.contains("✚ 2"));
        assert!(output.contains("to add"));
        assert!(output.contains("↻ 1"));
        assert!(output.contains("to change"));
    }
    
    #[test]
    fn test_format_html_output_apply_mode() {
        let mut plan = sample_terraform_plan();
        plan.mode = PlanMode::Apply;
        
        let output = format_html_output(&plan);
        assert!(output.contains("🌊 Terraform Apply Report"));
    }
    
    #[test]
    fn test_format_html_output_empty_plan() {
        let plan = TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary::default(),
            resources: Vec::new(),
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };
        
        let output = format_html_output(&plan);
        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("Terraform Plan Report"));
        // Should not contain summary items when all counts are 0
        assert!(!output.contains("✚ 0"));
        assert!(!output.contains("↻ 0"));
        assert!(!output.contains("✖ 0"));
    }
    
    #[test]
    fn test_format_html_output_with_destroy() {
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
        
        let output = format_html_output(&plan);
        assert!(output.contains("✚ 1"));
        assert!(output.contains("to add"));
        assert!(output.contains("✖ 2"));
        assert!(output.contains("to destroy"));
        assert!(output.contains("CREATE aws_s3_bucket.test"));
        assert!(output.contains("DESTROY aws_instance.legacy"));
    }
    
    #[test]
    fn test_format_html_output_with_read_resources() {
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
        
        let output = format_html_output(&plan);
        assert!(output.contains("⇐ 3"));
        assert!(output.contains("to read"));
    }
    
    #[test]
    fn test_format_html_output_interactive_features() {
        let plan = sample_terraform_plan();
        let output = format_html_output(&plan);
        
        // Check for interactive JavaScript features
        assert!(output.contains("<script>"));
        assert!(output.contains("toggleResource"));
        assert!(output.contains("filter-btn"));
        assert!(output.contains("searchBox"));
        assert!(output.contains("data-action"));
        
        // Check for CSS classes
        assert!(output.contains("class=\"resource create\""));
        assert!(output.contains("class=\"resource update\""));
        assert!(output.contains("resource-header"));
        assert!(output.contains("resource-toggle"));
        
        // Check for search and filter functionality
        assert!(output.contains("🔍 Search resources..."));
        assert!(output.contains("All Resources"));
        assert!(output.contains("✚ Create"));
        assert!(output.contains("↻ Update"));
        assert!(output.contains("✖ Destroy"));
        assert!(output.contains("⇐ Read"));
    }
    
    #[test]
    fn test_format_html_output_responsive_design() {
        let plan = sample_terraform_plan();
        let output = format_html_output(&plan);
        
        // Check for responsive CSS
        assert!(output.contains("@media (max-width: 768px)"));
        assert!(output.contains("viewport"));
        assert!(output.contains("grid-template-columns"));
    }
    
    #[test]
    fn test_format_html_output_accessibility() {
        let plan = sample_terraform_plan();
        let output = format_html_output(&plan);
        
        // Check for accessibility features
        assert!(output.contains("lang=\"en\""));
        assert!(output.contains("alt=") || output.contains("aria-") || !output.contains("<img"));
        // No images used, so alt text not required, but language is set
    }
    
    #[test]
    fn test_format_html_output_styling() {
        let plan = sample_terraform_plan();
        let output = format_html_output(&plan);
        
        // Check for modern styling features
        assert!(output.contains("linear-gradient"));
        assert!(output.contains("border-radius"));
        assert!(output.contains("box-shadow"));
        assert!(output.contains("transition"));
        assert!(output.contains("-apple-system"));
    }
    
    #[test]
    fn test_format_html_output_resources_with_attributes() {
        let plan = sample_terraform_plan();
        let output = format_html_output(&plan);
        
        // Check that resource attributes are properly displayed
        assert!(output.contains("attributes"));
        assert!(output.contains("<strong>"));
        assert!(output.contains("bucket"));
        assert!(output.contains("test-bucket"));
    }
}