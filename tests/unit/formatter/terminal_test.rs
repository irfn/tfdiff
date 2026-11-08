use crate::common::test_data::*;
use tfdiff::formatter::terminal::*;
use tfdiff::models::*;
use tfdiff::parser::cleaner::clean_ansi_codes;

#[cfg(test)]
mod terminal_formatter_tests {
    use super::*;

    #[test]
    fn test_format_terminal_output_basic() {
        let plan = sample_terraform_plan();
        let output = format_terminal_output(&plan);

        assert!(clean_ansi_codes(&output).contains("TERRAFORM PLAN ANALYSIS"));
        assert!(clean_ansi_codes(&output).contains("2 resources to add"));
        assert!(clean_ansi_codes(&output).contains("1 resources to change"));
        assert!(clean_ansi_codes(&output).contains("aws_s3_bucket.test"));
        assert!(clean_ansi_codes(&output).contains("aws_instance.web"));
    }

    #[test]
    fn test_format_terminal_output_apply_mode() {
        let mut plan = sample_terraform_plan();
        plan.mode = PlanMode::Apply;

        let output = format_terminal_output(&plan);
        assert!(clean_ansi_codes(&output).contains("TERRAFORM APPLY ANALYSIS"));
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
        assert!(clean_ansi_codes(&output).contains("TERRAFORM PLAN ANALYSIS"));
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
            resources: vec![sample_create_resource(), sample_destroy_resource()],
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata::default(),
        };

        let output = format_terminal_output(&plan);
        assert!(clean_ansi_codes(&output).contains("1 resources to add"));
        assert!(clean_ansi_codes(&output).contains("2 resources to destroy"));
        assert!(clean_ansi_codes(&output).contains("CREATE"));
        assert!(clean_ansi_codes(&output).contains("DESTROY"));
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
        assert!(clean_ansi_codes(&output).contains("3 data sources to read"));
    }

    #[test]
    fn test_format_resource_create() {
        let resource = sample_create_resource();
        let formatted = format_resource(&resource);

        assert!(clean_ansi_codes(&formatted).contains("CREATE"));
        assert!(clean_ansi_codes(&formatted).contains("aws_s3_bucket.test"));
        assert!(clean_ansi_codes(&formatted).contains("bucket = \"test-bucket\""));
        assert!(clean_ansi_codes(&formatted).contains("force_destroy = false"));
    }

    #[test]
    fn test_format_resource_update() {
        let resource = sample_update_resource();
        let formatted = format_resource(&resource);

        assert!(clean_ansi_codes(&formatted).contains("UPDATE"));
        assert!(clean_ansi_codes(&formatted).contains("aws_instance.web"));
        // The new format shows changes in diff format
        assert!(clean_ansi_codes(&formatted).contains("instance_type"));
        assert!(clean_ansi_codes(&formatted).contains("t3.small"));
    }

    #[test]
    fn test_format_resource_destroy() {
        let resource = sample_destroy_resource();
        let formatted = format_resource(&resource);

        assert!(clean_ansi_codes(&formatted).contains("DESTROY"));
        assert!(clean_ansi_codes(&formatted).contains("aws_instance.legacy"));
    }

    #[test]
    fn test_format_resource_no_op() {
        let mut resource = sample_create_resource();
        resource.action = ActionType::NoOp;

        let formatted = format_resource(&resource);
        assert!(clean_ansi_codes(&formatted).contains("NO-OP"));
    }

    #[test]
    fn test_format_resource_read() {
        let mut resource = sample_create_resource();
        resource.action = ActionType::Read;

        let formatted = format_resource(&resource);
        assert!(clean_ansi_codes(&formatted).contains("READ"));
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
        assert!(clean_ansi_codes(&formatted).contains("5 to add"));
        assert!(clean_ansi_codes(&formatted).contains("3 to change"));
        assert!(clean_ansi_codes(&formatted).contains("2 to destroy"));
        assert!(clean_ansi_codes(&formatted).contains("1 to read"));
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
        assert!(clean_ansi_codes(&formatted).contains("2 to add"));
        assert!(!clean_ansi_codes(&formatted).contains("to change"));
        assert!(clean_ansi_codes(&formatted).contains("1 to destroy"));
        assert!(!clean_ansi_codes(&formatted).contains("to read"));
    }

    fn update_resource(before: serde_json::Value, after: serde_json::Value) -> Resource {
        let mut resource = sample_update_resource();
        resource.changes = vec![Change {
            path: vec!["ip_addresses".to_string()],
            before: Some(before),
            after: Some(after),
            sensitive: false,
            computed: false,
        }];
        resource
    }

    #[test]
    fn test_format_resource_update_long_list_shows_only_changed_elements() {
        let before: Vec<String> = (0..283).map(|i| format!("10.0.0.{}/32", i)).collect();
        let mut after = before.clone();
        after.insert(10, "34.193.202.22/32".to_string());
        let formatted = format_resource(&update_resource(
            serde_json::json!(before),
            serde_json::json!(after),
        ));
        let plain = clean_ansi_codes(&formatted);

        assert!(plain.contains("  ~ ip_addresses\n"));
        assert!(plain.contains("    + \"34.193.202.22/32\"\n"));
        assert!(plain.contains("# (283 unchanged elements hidden)"));
        assert!(!plain.contains("10.0.0.1/32"));
        // Every line opens and closes its own colors
        for line in formatted.lines() {
            assert_eq!(line.matches("\x1b[").count() % 2, 0, "{:?}", line);
        }
    }

    #[test]
    fn test_format_resource_update_list_with_hidden_elements_placeholder() {
        // As produced by the text parser from Terraform's "# (283 unchanged elements hidden)"
        let formatted = format_resource(&update_resource(
            serde_json::json!(["(283 unchanged elements hidden)"]),
            serde_json::json!(["34.193.202.22/32", "(283 unchanged elements hidden)"]),
        ));
        let plain = clean_ansi_codes(&formatted);

        assert!(plain.contains("    + \"34.193.202.22/32\"\n"));
        assert!(plain.contains("# (283 unchanged elements hidden)"));
        assert!(!plain.contains("- "));
    }

    #[test]
    fn test_format_resource_update_map_shows_changed_keys() {
        let formatted = format_resource(&update_resource(
            serde_json::json!({"env": "dev", "owner": "a", "gone": "x"}),
            serde_json::json!({"env": "prod", "owner": "a", "new": "y"}),
        ));
        let plain = clean_ansi_codes(&formatted);

        assert!(plain.contains("~ env = \"dev\" → \"prod\""));
        assert!(plain.contains("- gone = \"x\""));
        assert!(plain.contains("+ new = \"y\""));
        assert!(plain.contains("# (1 unchanged attribute hidden)"));
    }

    #[test]
    fn test_format_resource_update_scalar_columns_align() {
        colored::control::set_override(true);
        let formatted = format_resource(&update_resource(
            serde_json::json!(false),
            serde_json::json!(true),
        ));
        colored::control::unset_override();
        let plain = clean_ansi_codes(&formatted);

        // Padding must not count the color codes, and values get the diff colors only
        assert!(plain.contains(&format!("    ─ {:<40} │ + true\n", "false")));
        assert!(!formatted.contains("\x1b[95m"));
    }
}
