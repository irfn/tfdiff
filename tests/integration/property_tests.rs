use proptest::prelude::*;
use tfdiff::parser::*;
use tfdiff::formatter::*;

#[cfg(test)]
mod proptest_integration {
    use super::*;
    
    proptest! {
        #[test]
        fn test_parser_doesnt_crash(input in any::<String>()) {
            // Parser should never panic, only return errors or results
            let _ = parse_terraform_output(&input);
        }
        
        #[test]
        fn test_clean_function_is_idempotent(input in any::<String>()) {
            let cleaned_once = clean_input(&input).unwrap_or_default();
            let cleaned_twice = clean_input(&cleaned_once).unwrap_or_default();
            prop_assert_eq!(cleaned_once, cleaned_twice);
        }
        
        #[test]
        fn test_ansi_cleaning_removes_all_codes(
            input in prop::string::string_regex(r".*\x1b\[[0-9;]*[mK].*").unwrap()
        ) {
            let cleaned = clean_ansi_codes(&input);
            prop_assert!(!cleaned.contains('\x1b'));
        }
        
        #[test]
        fn test_summary_parsing_with_random_numbers(
            add in 0u32..1000,
            change in 0u32..1000,
            destroy in 0u32..1000
        ) {
            let plan_line = format!("Plan: {} to add, {} to change, {} to destroy.", add, change, destroy);
            let lines = vec![plan_line.as_str()];
            
            if let Ok(summary) = extract_summary(&lines) {
                prop_assert_eq!(summary.add as u32, add);
                prop_assert_eq!(summary.change as u32, change);
                prop_assert_eq!(summary.destroy as u32, destroy);
            }
        }
        
        #[test]
        fn test_apply_summary_parsing_with_random_numbers(
            added in 0u32..1000,
            changed in 0u32..1000,
            destroyed in 0u32..1000
        ) {
            let apply_line = format!("Apply complete! Resources: {} added, {} changed, {} destroyed.", added, changed, destroyed);
            let lines = vec![apply_line.as_str()];
            
            if let Ok(summary) = extract_summary(&lines) {
                prop_assert_eq!(summary.add as u32, added);
                prop_assert_eq!(summary.change as u32, changed);
                prop_assert_eq!(summary.destroy as u32, destroyed);
            }
        }
        
        #[test]
        fn test_json_formatter_produces_valid_json(
            add in 0usize..100,
            change in 0usize..100,
            destroy in 0usize..100
        ) {
            use tfdiff::models::*;
            
            let plan = TerraformPlan {
                mode: PlanMode::Plan,
                summary: Summary { add, change, destroy, read: 0 },
                resources: Vec::new(),
                data_sources: Vec::new(),
                warnings: Vec::new(),
                metadata: Metadata::default(),
            };
            
            if let Ok(json_output) = format_json_output(&plan) {
                // Should always produce valid JSON
                prop_assert!(serde_json::from_str::<serde_json::Value>(&json_output).is_ok());
            }
        }
        
        #[test]
        fn test_markdown_formatter_produces_valid_markdown(
            add in 0usize..50,
            change in 0usize..50,
            destroy in 0usize..50
        ) {
            use tfdiff::models::*;
            
            let plan = TerraformPlan {
                mode: PlanMode::Plan,
                summary: Summary { add, change, destroy, read: 0 },
                resources: Vec::new(),
                data_sources: Vec::new(),
                warnings: Vec::new(),
                metadata: Metadata::default(),
            };
            
            let markdown_output = format_markdown_output(&plan);
            
            // Basic markdown structure checks
            prop_assert!(markdown_output.contains("# Terraform Plan Report"));
            prop_assert!(markdown_output.contains("## Summary"));
            let add_str = format!("**{}** resources to add", add);
            let change_str = format!("**{}** resources to change", change);
            let destroy_str = format!("**{}** resources to destroy", destroy);
            prop_assert!(markdown_output.contains(&add_str));
            prop_assert!(markdown_output.contains(&change_str));
            prop_assert!(markdown_output.contains(&destroy_str));
        }
        
        #[test]
        fn test_html_formatter_produces_valid_html_structure(
            add in 0usize..30,
            change in 0usize..30
        ) {
            use tfdiff::models::*;
            
            let plan = TerraformPlan {
                mode: PlanMode::Plan,
                summary: Summary { add, change, destroy: 0, read: 0 },
                resources: Vec::new(),
                data_sources: Vec::new(),
                warnings: Vec::new(),
                metadata: Metadata::default(),
            };
            
            let html_output = format_html_output(&plan);
            
            // Basic HTML structure checks
            prop_assert!(html_output.contains("<!DOCTYPE html>"));
            prop_assert!(html_output.contains("<html"));
            prop_assert!(html_output.contains("</html>"));
            prop_assert!(html_output.contains("<head>"));
            prop_assert!(html_output.contains("<body>"));
            if add > 0 {
                let add_str = format!(">{}<", add);
                prop_assert!(html_output.contains(&add_str));
                prop_assert!(html_output.contains("to add"));
            }
            if change > 0 {
                let change_str = format!(">{}<", change);
                prop_assert!(html_output.contains(&change_str));
                prop_assert!(html_output.contains("to change"));
            }
        }
        
        #[test]
        fn test_spinner_cleaning_removes_all_spinners(
            input_prefix in "[a-zA-Z0-9 ]*",
            input_suffix in "[a-zA-Z0-9 ]*"
        ) {
            let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            
            for &spinner in &spinner_chars {
                let input = format!("{}{}{}", input_prefix, spinner, input_suffix);
                let cleaned = clean_spinner_chars(&input);
                prop_assert!(!cleaned.contains(spinner), "Failed to remove spinner: {}", spinner);
            }
        }
        
        #[test]
        fn test_cdk_prefix_cleaning(
            prefix in "base14-cd[a-zA-Z0-9-]*",
            suffix in "[a-zA-Z0-9._-]*"
        ) {
            let input = format!("{}.{}", prefix, suffix);
            let cleaned = clean_cdk_prefixes(&input);
            
            // Should remove the CDK prefix
            prop_assert!(!cleaned.starts_with("base14-cd"));
            let expected_suffix = format!(".{}", suffix);
            prop_assert!(cleaned.contains(&expected_suffix));
        }
        
        #[test]
        fn test_resource_parsing_stability(
            resource_type in "[a-zA-Z][a-zA-Z0-9_]*",
            provider in "[a-zA-Z][a-zA-Z0-9_]*", 
            resource_name in "[a-zA-Z][a-zA-Z0-9_]*"
        ) {
            let line1 = format!("# {}.{}.{} will be created", resource_type, provider, resource_name);
            let line2 = format!("+ resource \"{}\" \"{}\" {{", resource_type, resource_name);
            let resource_lines = vec![
                line1.as_str(),
                line2.as_str(),
                "+   test_attr = \"value\"",
                "  }",
            ];
            
            let resources = parse_resource_changes(&resource_lines);
            
            if !resources.is_empty() {
                prop_assert_eq!(&resources[0].type_name, &resource_type);
                prop_assert_eq!(&resources[0].provider, &provider);
                prop_assert_eq!(&resources[0].name, &resource_name);
            }
        }
    }
}