use serde_json::{json, Value};
use tfdiff::formatter::*;
use tfdiff::models::*;
use tfdiff::parser::*;

#[cfg(test)]
mod json_plan_tests {
    use super::*;
    use crate::common::assertions::{assert_summary_totals, find_resource};
    use crate::common::fixtures::load_terraform_fixture;

    const SECRETS: [&str; 7] = [
        "VARIABLE-SECRET-123",
        "BECAME-SENSITIVE-OLD",
        "BECAME-SENSITIVE-NEW",
        "CREATE-SECRET-456",
        "UPDATE-OLD-SECRET",
        "UPDATE-NEW-SECRET",
        "DELETE-SECRET-789",
    ];

    fn fixture_plan() -> TerraformPlan {
        parse_terraform_output(&load_terraform_fixture("show_json_plan.json")).unwrap()
    }

    /// Minimal plan document with the given resource changes.
    fn plan_with(resource_changes: Vec<Value>) -> TerraformPlan {
        let plan = json!({
            "format_version": "1.2",
            "planned_values": {},
            "resource_changes": resource_changes,
        });
        parse_terraform_output(&plan.to_string()).unwrap()
    }

    /// A resource change with only the fields tfdiff reads.
    fn resource_change(address: &str, actions: &[&str], before: Value, after: Value) -> Value {
        json!({
            "address": address,
            "type": "aws_instance",
            "name": "x",
            "provider_name": "aws",
            "change": {"actions": actions, "before": before, "after": after},
        })
    }

    fn changed_paths(resource: &Resource) -> Vec<&str> {
        resource
            .changes
            .iter()
            .map(|c| c.path[0].as_str())
            .collect()
    }

    /// Both sides replaced by the sensitive placeholder.
    fn assert_masked(change: &Change) {
        assert_eq!(change.before, Some(json!(SENSITIVE_VALUE)));
        assert_eq!(change.after, Some(json!(SENSITIVE_VALUE)));
        assert!(change.sensitive);
    }

    fn change<'a>(resource: &'a Resource, attr: &str) -> Option<&'a Change> {
        resource.changes.iter().find(|c| c.path == vec![attr])
    }

    #[test]
    fn test_is_json_input() {
        assert!(is_json_input("  \n{\"format_version\": \"1.2\"}"));
        assert!(!is_json_input(
            "Terraform will perform the following actions:"
        ));
        assert!(!is_json_input(""));
    }

    #[test]
    fn test_summary_and_metadata() {
        let plan = fixture_plan();

        assert_eq!(plan.mode, PlanMode::Plan);
        // Replacements count as both an add and a destroy, like Terraform's "Plan:" line
        assert_summary_totals(&plan.summary, 3, 1, 3);
        assert_eq!(plan.summary.read, 1);
        assert_eq!(plan.metadata.terraform_version.as_deref(), Some("1.9.5"));
        assert_eq!(
            plan.metadata.timestamp.as_deref(),
            Some("2026-10-31T12:00:00Z")
        );
    }

    #[test]
    fn test_actions_and_addresses() {
        let plan = fixture_plan();

        let actions: Vec<(&str, ActionType)> = plan
            .resources
            .iter()
            .map(|r| (r.id.as_str(), r.action.clone()))
            .collect();
        assert_eq!(
            actions,
            vec![
                ("aws_db_instance.main", ActionType::Create),
                ("module.app.aws_instance.web[0]", ActionType::Update),
                ("aws_iam_user.legacy", ActionType::Destroy),
                ("aws_instance.db[\"primary\"]", ActionType::Replace),
                ("aws_lb.front", ActionType::Replace),
                ("data.http.health", ActionType::Read),
            ]
        );

        let web = find_resource(&plan, "module.app.aws_instance.web[0]");
        assert_eq!(web.type_name, "aws_instance");
        assert_eq!(web.name, "web");
        assert_eq!(web.provider, "registry.terraform.io/hashicorp/aws");
    }

    #[test]
    fn test_create_attributes() {
        let plan = fixture_plan();
        let db = find_resource(&plan, "aws_db_instance.main");

        assert!(db.changes.is_empty());
        assert_eq!(db.attributes["engine"], json!("postgres"));
        assert_eq!(db.attributes["allocated_storage"], json!(20));
        assert_eq!(db.attributes["deletion_protection"], json!(true));
        assert_eq!(db.attributes["arn"], json!("(known after apply)"));
        assert_eq!(
            db.attributes["tags_all"],
            json!({"Name": "(known after apply)"})
        );
        assert_eq!(
            db.attributes["tags"],
            json!({"Name": "main-db", "Team": "platform"})
        );
        assert_eq!(db.attributes["password"], json!("(sensitive value)"));
        assert!(!db.attributes.contains_key("parameter_group_name"));
    }

    #[test]
    fn test_update_changes() {
        let plan = fixture_plan();
        let web = find_resource(&plan, "module.app.aws_instance.web[0]");

        assert!(web.attributes.is_empty());
        let paths = changed_paths(web);
        assert_eq!(
            paths,
            vec![
                "admin_password",
                "instance_type",
                "monitoring",
                "public_ip",
                "tags",
                "user_data"
            ]
        );

        let instance_type = change(web, "instance_type").unwrap();
        assert_eq!(instance_type.before, Some(json!("t3.micro")));
        assert_eq!(instance_type.after, Some(json!("t3.small")));
        assert!(!instance_type.sensitive && !instance_type.computed);

        let monitoring = change(web, "monitoring").unwrap();
        assert_eq!(monitoring.before, Some(json!(false)));
        assert_eq!(monitoring.after, None);

        let public_ip = change(web, "public_ip").unwrap();
        assert_eq!(public_ip.before, Some(json!("203.0.113.10")));
        assert_eq!(public_ip.after, Some(json!("(known after apply)")));
        assert!(public_ip.computed);

        let user_data = change(web, "user_data").unwrap();
        assert_masked(user_data);

        // Only sensitive after the change: the old value must be masked as well
        let admin_password = change(web, "admin_password").unwrap();
        assert_masked(admin_password);

        assert!(change(web, "id").is_none());
        assert!(change(web, "security_groups").is_none());
    }

    #[test]
    fn test_destroy_masks_sensitive_list_element() {
        let plan = fixture_plan();
        let user = find_resource(&plan, "aws_iam_user.legacy");

        assert_eq!(user.attributes["name"], json!("legacy-user"));
        assert_eq!(
            user.attributes["access_keys"],
            json!(["AKIAEXAMPLE", "(sensitive value)"])
        );
    }

    #[test]
    fn test_replace_changes() {
        let plan = fixture_plan();
        let db = find_resource(&plan, "aws_instance.db[\"primary\"]");

        let paths = changed_paths(db);
        assert_eq!(paths, vec!["ami", "id"]);
        assert_eq!(
            change(db, "id").unwrap().after,
            Some(json!("(known after apply)"))
        );
    }

    #[test]
    fn test_read_attributes() {
        let plan = fixture_plan();
        let health = find_resource(&plan, "data.http.health");

        assert_eq!(
            health.attributes["url"],
            json!("https://example.com/health")
        );
        assert_eq!(health.attributes["body"], json!("(known after apply)"));
        assert!(!health.attributes.contains_key("request_headers"));
    }

    #[test]
    fn test_no_secrets_in_any_output_format() {
        let plan = fixture_plan();
        let outputs = vec![
            format_terminal_output(&plan),
            format_json_output(&plan).unwrap(),
            format_html_output(&plan),
            format_markdown_output(&plan),
        ];

        for output in outputs {
            for secret in SECRETS {
                assert!(!output.contains(secret), "leaked {}", secret);
            }
        }
    }

    #[test]
    fn test_empty_plan() {
        let plan = plan_with(vec![]);

        assert!(plan.resources.is_empty());
        assert!(plan.warnings.is_empty());
        assert_summary_totals(&plan.summary, 0, 0, 0);
    }

    #[test]
    fn test_byte_order_mark_is_ignored() {
        let input = format!("\u{feff}{}", load_terraform_fixture("show_json_plan.json"));
        let plan = parse_terraform_output(&input).unwrap();

        assert_eq!(plan.summary.add, 3);
    }

    #[test]
    fn test_invalid_json_is_an_error() {
        assert!(parse_terraform_output("{ not json").is_err());
        assert!(parse_terraform_output(r#"{"resource_changes": []}"#).is_err());
    }

    #[test]
    fn test_state_json_is_an_error() {
        // `terraform show -json` without a plan file prints the state
        let state = r#"{"format_version": "1.0", "terraform_version": "1.9.5", "values": {}}"#;
        let err = parse_terraform_output(state).unwrap_err().to_string();

        assert!(err.contains("not a plan"), "{}", err);
    }

    #[test]
    fn test_plan_json_lines_use_text_parser() {
        // `terraform plan -json` streams JSON lines; the text parser reads its summary message
        let input = concat!(
            r#"{"@level":"info","@message":"Terraform 1.9.5","type":"version"}"#,
            "\n",
            r#"{"@level":"info","@message":"Plan: 1 to add, 2 to change, 0 to destroy.","type":"change_summary"}"#,
        );

        assert!(!is_json_input(input));
        let plan = parse_terraform_output(input).unwrap();
        assert_eq!(plan.summary.add, 1);
        assert_eq!(plan.summary.change, 2);
    }

    #[test]
    fn test_errored_plan_has_warning() {
        let input = r#"{"format_version": "1.2", "planned_values": {}, "errored": true}"#;
        let plan = parse_terraform_output(input).unwrap();

        assert_eq!(plan.warnings.len(), 1);
        assert_eq!(plan.warnings[0].level, WarningLevel::Error);
    }

    #[test]
    fn test_deposed_object_has_distinct_id() {
        let mut deposed = resource_change(
            "aws_instance.x",
            &["delete"],
            json!({"ami": "a"}),
            Value::Null,
        );
        deposed["deposed"] = json!("abc123");
        let current = resource_change(
            "aws_instance.x",
            &["create"],
            Value::Null,
            json!({"ami": "b"}),
        );
        let plan = plan_with(vec![deposed, current]);

        let ids: Vec<&str> = plan.resources.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            vec!["aws_instance.x (deposed object abc123)", "aws_instance.x"]
        );
    }

    #[test]
    fn test_html_escapes_addresses_and_keys() {
        let plan = plan_with(vec![
            resource_change(
                r#"aws_iam_user.u["<img src=x onerror=alert(1)>"]"#,
                &["create"],
                Value::Null,
                json!({"tags": {"<svg onload=alert(2)>": "v"}}),
            ),
            resource_change(
                "aws_iam_user.v",
                &["update"],
                json!({"<b>old</b>": "1"}),
                json!({"<b>old</b>": "2"}),
            ),
        ]);
        let html = format_html_output(&plan);

        assert!(!html.contains("<img"));
        assert!(!html.contains("<svg"));
        assert!(!html.contains("<b>"));
        assert!(html.contains("&lt;img src=x onerror=alert(1)&gt;"));
    }

    #[test]
    fn test_terminal_handles_long_non_ascii_values() {
        let plan = plan_with(vec![resource_change(
            "aws_instance.x",
            &["update"],
            json!({"description": "é".repeat(45)}),
            json!({"description": "ü"}),
        )]);
        let output = format_terminal_output(&plan);

        assert!(output.contains("..."));
    }

    #[test]
    fn test_root_level_sensitive_marker_masks_everything() {
        let mut rc = resource_change(
            "aws_instance.x",
            &["update"],
            json!({"v": "ROOT-OLD"}),
            json!({"v": "ROOT-NEW"}),
        );
        rc["change"]["before_sensitive"] = json!(true);
        rc["change"]["after_sensitive"] = json!(true);
        let plan = plan_with(vec![rc]);

        let v = change(&plan.resources[0], "v").unwrap();
        assert_masked(v);
    }

    #[test]
    fn test_sensitive_value_that_changes_shape_is_masked() {
        let mut rc = resource_change(
            "aws_instance.x",
            &["update"],
            json!({"v": {"k": "SHAPE-OLD"}}),
            json!({"v": ["SHAPE-NEW"]}),
        );
        rc["change"]["before_sensitive"] = json!({"v": {"k": true}});
        rc["change"]["after_sensitive"] = json!({"v": [true]});
        let plan = plan_with(vec![rc]);

        let v = change(&plan.resources[0], "v").unwrap();
        assert_masked(v);
    }

    #[test]
    fn test_unsupported_actions_produce_warning() {
        let forget = resource_change("aws_instance.a", &["forget"], json!({}), Value::Null);
        let mut import = resource_change("aws_instance.b", &["no-op"], json!({}), json!({}));
        import["change"]["importing"] = json!({"id": "i-123"});
        let unchanged = resource_change("aws_instance.c", &["no-op"], json!({}), json!({}));
        let plan = plan_with(vec![forget, import, unchanged]);

        assert!(plan.resources.is_empty());
        assert_eq!(plan.warnings.len(), 1);
        assert_eq!(plan.warnings[0].level, WarningLevel::Warning);
        assert!(plan.warnings[0].message.ends_with("forget, import"));
    }

    #[test]
    fn test_markdown_shows_update_changes() {
        let markdown = format_markdown_output(&fixture_plan());

        assert!(markdown.contains("- instance_type: \"t3.micro\"\n+ instance_type: \"t3.small\""));
        assert!(markdown.contains("- name: \"front-old\"\n+ name: \"front-new\""));
    }

    #[test]
    fn test_existing_plan_json_fixture() {
        let plan = parse_terraform_output(&load_terraform_fixture("plan.json")).unwrap();

        assert_eq!(plan.summary.add, 1);
        assert_eq!(plan.summary.change, 1);
        let web = find_resource(&plan, "aws_instance.web_server");
        assert_eq!(
            change(web, "instance_type").unwrap().after,
            Some(json!("t3.small"))
        );
    }
}
