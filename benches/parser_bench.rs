use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tfdiff::parser::*;
use tfdiff::formatter::*;
// Note: Benchmarks will need fixtures loaded differently
// For now, we'll create inline test data

fn parser_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    
    // Simple inline test data
    let simple_plan = r#"
Terraform used the selected providers to generate the following execution plan.

# aws_s3_bucket.test will be created
+ resource "aws_s3_bucket" "test" {
    + bucket = "test-bucket"
    + id     = (known after apply)
  }

Plan: 1 to add, 0 to change, 0 to destroy.
    "#;
    
    let complex_update = r#"
Terraform will perform the following actions:

# aws_instance.web will be updated in-place
~ resource "aws_instance" "web" {
    ~ instance_type = "t2.micro" -> "t3.small"
    ~ tags = {
        ~ "Environment" = "staging" -> "production"
      }
  }

Plan: 0 to add, 1 to change, 0 to destroy.
    "#;
    
    group.bench_function("simple_plan", |b| {
        b.iter(|| parse_terraform_output(black_box(simple_plan)))
    });
    
    group.bench_function("complex_update", |b| {
        b.iter(|| parse_terraform_output(black_box(complex_update)))
    });
    
    group.finish();
}

fn cleaner_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cleaner");
    
    // Test ANSI cleaning performance
    let ansi_input = "\x1b[32m+ resource\x1b[0m \x1b[31m- old\x1b[0m".repeat(100);
    group.bench_function("ansi_cleaning", |b| {
        b.iter(|| clean_ansi_codes(black_box(&ansi_input)))
    });
    
    // Test spinner cleaning
    let spinner_input = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏".repeat(100);
    group.bench_function("spinner_cleaning", |b| {
        b.iter(|| clean_spinner_chars(black_box(&spinner_input)))
    });
    
    // Test CDK prefix cleaning
    let cdk_input = "base14-cd-staging base14-cd-prod base14-cd-test".repeat(50);
    group.bench_function("cdk_prefix_cleaning", |b| {
        b.iter(|| clean_cdk_prefixes(black_box(&cdk_input)))
    });
    
    group.finish();
}

fn formatter_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatter");
    
    // Create simple test plan
    use tfdiff::models::*;
    use std::collections::HashMap;
    
    let mut attributes = HashMap::new();
    attributes.insert("bucket".to_string(), serde_json::Value::String("test-bucket".to_string()));
    
    let resource = Resource {
        id: "aws_s3_bucket.test".to_string(),
        name: "test".to_string(),
        type_name: "aws_s3_bucket".to_string(),
        provider: "aws".to_string(),
        action: ActionType::Create,
        changes: Vec::new(),
        attributes,
        applied: false,
    };
    
    let small_plan = TerraformPlan {
        mode: PlanMode::Plan,
        summary: Summary { add: 1, change: 0, destroy: 0, read: 0 },
        resources: vec![resource],
        data_sources: Vec::new(),
        warnings: Vec::new(),
        metadata: Metadata::default(),
    };
    
    group.bench_function("terminal_small", |b| {
        b.iter(|| format_terminal_output(black_box(&small_plan)))
    });
    
    group.bench_function("json_small", |b| {
        b.iter(|| format_json_output(black_box(&small_plan)))
    });
    
    group.bench_function("html_small", |b| {
        b.iter(|| format_html_output(black_box(&small_plan)))
    });
    
    group.bench_function("markdown_small", |b| {
        b.iter(|| format_markdown_output(black_box(&small_plan)))
    });
    
    group.finish();
}

fn end_to_end_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");
    
    let simple_plan = "Plan: 1 to add, 0 to change, 0 to destroy.";
    
    group.bench_function("parse_and_format_terminal", |b| {
        b.iter(|| {
            let parsed = parse_terraform_output(black_box(simple_plan)).unwrap();
            format_terminal_output(black_box(&parsed))
        })
    });
    
    group.bench_function("parse_and_format_json", |b| {
        b.iter(|| {
            let parsed = parse_terraform_output(black_box(simple_plan)).unwrap();
            format_json_output(black_box(&parsed)).unwrap()
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    parser_benchmarks,
    cleaner_benchmarks,
    formatter_benchmarks,
    end_to_end_benchmarks
);
criterion_main!(benches);