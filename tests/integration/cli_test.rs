use crate::common::fixtures::{fixture_path, load_fixture};
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[cfg(test)]
mod cli_integration_tests {
    use super::*;

    fn create_temp_file_with_content(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Beautiful Terraform plan"))
            .stdout(predicate::str::contains("--format"));
    }

    #[test]
    fn test_cli_version() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains("tfdiff"));
    }

    #[test]
    fn test_cli_with_file_input() {
        let plan_content = r#"
Terraform used the selected providers to generate the following execution plan.

# aws_s3_bucket.test will be created
+ resource "aws_s3_bucket" "test" {
    + bucket = "test-bucket"
    + id     = (known after apply)
  }

Plan: 1 to add, 0 to change, 0 to destroy.
        "#;

        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("TERRAFORM PLAN ANALYSIS"))
            .stdout(predicate::str::contains("1 resources to add"));
    }

    #[test]
    fn test_cli_json_format() {
        let plan_content = "Plan: 2 to add, 1 to change, 0 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .arg("--format")
            .arg("json")
            .assert()
            .success()
            .stdout(predicate::str::contains("mode"))
            .stdout(predicate::str::contains("summary"));
    }

    #[test]
    fn test_cli_html_format() {
        let plan_content = "Plan: 1 to add, 0 to change, 1 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .arg("--format")
            .arg("html")
            .assert()
            .success()
            .stdout(predicate::str::contains("<!DOCTYPE html>"))
            .stdout(predicate::str::contains(
                "<title>Terraform Plan - Infrastructure</title>",
            ));
    }

    #[test]
    fn test_cli_markdown_format() {
        let plan_content = "Plan: 0 to add, 2 to change, 0 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .arg("--format")
            .arg("markdown")
            .assert()
            .success()
            .stdout(predicate::str::contains("# Terraform Plan"));
    }

    #[test]
    fn test_cli_web_mode() {
        let plan_content = "Plan: 1 to add, 1 to change, 1 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .arg("--web")
            .arg("--port")
            .arg("9999")
            .assert()
            .success()
            .stderr(predicate::str::contains("Web UI mode on port 9999"));
    }

    #[test]
    fn test_cli_filter_option() {
        let plan_content = "Plan: 1 to add, 1 to change, 1 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .arg("--filter")
            .arg("create,update")
            .assert()
            .success();
    }

    #[test]
    fn test_cli_invalid_filter_fails() {
        let plan_content = "Plan: 1 to add, 1 to change, 1 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .args(["--filter", "create,bogus"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("invalid value 'bogus'"));
    }

    #[test]
    fn test_cli_help_lists_filters() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg("-h")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "create, update, destroy, replace, read, noop",
            ));
    }

    #[test]
    fn test_cli_verbose_mode() {
        let plan_content = "Plan: 1 to add, 0 to change, 0 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path()).arg("--verbose").assert().success();
    }

    #[test]
    fn test_cli_quiet_mode() {
        let plan_content = "Plan: 1 to add, 0 to change, 0 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path()).arg("--quiet").assert().success();
    }

    #[test]
    fn test_cli_summary_mode() {
        let plan_content = "Plan: 5 to add, 3 to change, 2 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path()).arg("--summary").assert().success();
    }

    #[test]
    fn test_cli_watch_mode() {
        let plan_content = "Plan: 1 to add, 0 to change, 0 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path()).arg("--watch").assert().success();
    }

    #[test]
    fn test_cli_nonexistent_file() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg("/nonexistent/file.txt")
            .assert()
            .failure() // File not found should return error
            .stderr(predicate::str::contains("No such file or directory"));
    }

    #[test]
    fn test_cli_stdin_mode() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.write_stdin("Plan: 0 to add, 0 to change, 0 to destroy.")
            .assert()
            .success()
            .stdout(predicate::str::contains("TERRAFORM PLAN ANALYSIS"));
    }

    #[test]
    fn test_cli_invalid_format() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg("--format").arg("invalid").assert().failure();
    }

    #[test]
    fn test_cli_invalid_port() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg("--port").arg("invalid").assert().failure();
    }

    #[test]
    fn test_cli_conflicting_options() {
        let plan_content = "Plan: 1 to add, 0 to change, 0 to destroy.";
        let file = create_temp_file_with_content(plan_content);

        // Quiet and verbose should both be accepted (implementation decides precedence)
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .arg("--quiet")
            .arg("--verbose")
            .assert()
            .success();
    }

    #[test]
    fn test_cli_browser_mode() {
        let plan_content = r#"
Terraform used the selected providers to generate the following execution plan.

# aws_s3_bucket.test will be created
+ resource "aws_s3_bucket" "test" {
    + bucket = "test-bucket"
    + id     = (known after apply)
  }

Plan: 1 to add, 0 to change, 0 to destroy.
        "#;
        let file = create_temp_file_with_content(plan_content);

        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(file.path())
            .arg("--browser")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "🌐 Opened Terraform diff in browser",
            ));
    }

    #[test]
    fn test_cli_browser_flag_available() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("--browser"))
            .stdout(predicate::str::contains(
                "Generate HTML and open in browser",
            ));
    }

    const JSON_PLAN_FIXTURE: &str = "terraform/show_json_plan.json";

    #[test]
    fn test_cli_json_plan_file_input() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.arg(fixture_path(JSON_PLAN_FIXTURE))
            .assert()
            .success()
            .stdout(predicate::str::contains("module.app.aws_instance.web[0]"))
            .stdout(predicate::str::contains("REPLACE"))
            .stdout(predicate::str::contains("CREATE-SECRET-456").not());
    }

    #[test]
    fn test_cli_json_plan_stdin() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.write_stdin(load_fixture(JSON_PLAN_FIXTURE))
            .arg("--summary")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Summary: 3 to add, 1 to change, 3 to destroy",
            ));
    }

    #[test]
    fn test_cli_json_plan_filter_replace() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        let output = cmd
            .arg(fixture_path(JSON_PLAN_FIXTURE))
            .args(["--format", "json", "--filter", "replace"])
            .output()
            .unwrap();
        assert!(output.status.success());

        let plan: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let ids: Vec<&str> = plan["resources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap())
            .collect();
        assert_eq!(ids, vec!["aws_instance.db[\"primary\"]", "aws_lb.front"]);
    }

    #[test]
    fn test_cli_invalid_json_input() {
        let mut cmd = cargo_bin_cmd!("tfdiff");
        cmd.write_stdin("{ not json")
            .assert()
            .failure()
            .stderr(predicate::str::contains("terraform show -json"));
    }
}
