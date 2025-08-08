# Terraform Diff CLI - Comprehensive Testing Strategy

## Testing Requirements

### Target Coverage
- **Overall Coverage**: Minimum 90%
- **Core Parser**: 95%+ coverage
- **Critical Path**: 100% coverage for diff parsing and resource detection
- **Integration Tests**: Cover all CLI commands and options

### Testing Pyramid
```
         /\
        /  \    E2E Tests (10%)
       /────\   - Full CLI execution
      /      \  - Web UI interaction
     /────────\ Integration Tests (30%)
    /          \- Parser + Formatter
   /────────────\Component Tests (40%)
  /              \- Individual modules
 /────────────────\Unit Tests (20%)
/                  \- Pure functions
```

## Test Structure

### Directory Layout
```
tests/
├── unit/
│   ├── parser/
│   │   ├── cleaner_test.rs
│   │   ├── terraform_test.rs
│   │   ├── diff_test.rs
│   │   └── json_parser_test.rs
│   ├── formatter/
│   │   ├── terminal_test.rs
│   │   ├── html_test.rs
│   │   ├── json_test.rs
│   │   └── markdown_test.rs
│   └── models/
│       └── serialization_test.rs
├── integration/
│   ├── cli_test.rs
│   ├── file_processing_test.rs
│   ├── web_server_test.rs
│   └── watch_mode_test.rs
├── e2e/
│   ├── terraform_versions_test.rs
│   ├── large_plan_test.rs
│   ├── error_handling_test.rs
│   └── performance_test.rs
├── fixtures/
│   ├── terraform/
│   │   ├── v0.12/
│   │   ├── v0.13/
│   │   ├── v0.14/
│   │   ├── v0.15/
│   │   ├── v1.0/
│   │   ├── v1.5/
│   │   └── latest/
│   ├── providers/
│   │   ├── aws/
│   │   ├── azure/
│   │   ├── gcp/
│   │   └── kubernetes/
│   ├── edge_cases/
│   │   ├── massive_plan.json
│   │   ├── unicode_content.json
│   │   ├── nested_modules.json
│   │   └── circular_dependencies.json
│   └── malformed/
│       ├── incomplete_plan.txt
│       ├── corrupted_json.txt
│       └── mixed_encoding.txt
└── benchmarks/
    ├── parser_bench.rs
    ├── formatter_bench.rs
    └── memory_bench.rs
```

## Realistic Terraform Test Fixtures

### 1. Basic AWS Infrastructure Plan
```json
{
  "format_version": "1.2",
  "terraform_version": "1.5.7",
  "planned_values": {
    "root_module": {
      "resources": [
        {
          "address": "aws_s3_bucket.app_bucket",
          "mode": "managed",
          "type": "aws_s3_bucket",
          "name": "app_bucket",
          "provider_name": "registry.terraform.io/hashicorp/aws",
          "schema_version": 0,
          "values": {
            "bucket": "my-app-bucket-prod-2024",
            "bucket_prefix": null,
            "force_destroy": false,
            "tags": {
              "Environment": "production",
              "ManagedBy": "terraform",
              "Application": "web-app"
            },
            "tags_all": {
              "Environment": "production",
              "ManagedBy": "terraform",
              "Application": "web-app",
              "Organization": "acme-corp"
            }
          }
        }
      ]
    }
  },
  "resource_changes": [
    {
      "address": "aws_s3_bucket.app_bucket",
      "mode": "managed",
      "type": "aws_s3_bucket",
      "name": "app_bucket",
      "provider_name": "registry.terraform.io/hashicorp/aws",
      "change": {
        "actions": ["create"],
        "before": null,
        "after": {
          "bucket": "my-app-bucket-prod-2024",
          "bucket_prefix": null,
          "force_destroy": false,
          "tags": {
            "Environment": "production",
            "ManagedBy": "terraform",
            "Application": "web-app"
          }
        },
        "after_unknown": {
          "acceleration_status": true,
          "acl": true,
          "arn": true,
          "bucket_domain_name": true,
          "bucket_regional_domain_name": true,
          "cors_rule": true,
          "grant": true,
          "hosted_zone_id": true,
          "id": true,
          "lifecycle_rule": true,
          "logging": true,
          "object_lock_configuration": true,
          "object_lock_enabled": true,
          "policy": true,
          "region": true,
          "replication_configuration": true,
          "request_payer": true,
          "server_side_encryption_configuration": true,
          "tags": {},
          "tags_all": {
            "Organization": true
          },
          "versioning": true,
          "website": true,
          "website_domain": true,
          "website_endpoint": true
        }
      }
    }
  ],
  "configuration": {
    "provider_config": {
      "aws": {
        "name": "aws",
        "full_name": "registry.terraform.io/hashicorp/aws",
        "version_constraint": "~> 5.0",
        "expressions": {
          "region": {
            "constant_value": "us-east-1"
          }
        }
      }
    }
  }
}
```

### 2. Complex Update Scenario
```json
{
  "resource_changes": [
    {
      "address": "aws_instance.web_server",
      "mode": "managed",
      "type": "aws_instance",
      "name": "web_server",
      "change": {
        "actions": ["update"],
        "before": {
          "ami": "ami-0c55b159cbfafe1f0",
          "instance_type": "t2.micro",
          "tags": {
            "Name": "WebServer",
            "Environment": "staging"
          },
          "subnet_id": "subnet-12345678",
          "vpc_security_group_ids": ["sg-abcdef12"]
        },
        "after": {
          "ami": "ami-0d70546e43a941d70",
          "instance_type": "t3.small",
          "tags": {
            "Name": "WebServer",
            "Environment": "production",
            "Version": "2.0"
          },
          "subnet_id": "subnet-12345678",
          "vpc_security_group_ids": ["sg-abcdef12", "sg-fedcba21"]
        },
        "after_sensitive": {
          "user_data": true
        },
        "before_sensitive": {
          "user_data": true
        }
      }
    }
  ]
}
```

### 3. Module-based Infrastructure
```json
{
  "planned_values": {
    "root_module": {
      "child_modules": [
        {
          "address": "module.vpc",
          "resources": [
            {
              "address": "module.vpc.aws_vpc.main",
              "mode": "managed",
              "type": "aws_vpc",
              "name": "main",
              "provider_name": "registry.terraform.io/hashicorp/aws",
              "values": {
                "cidr_block": "10.0.0.0/16",
                "enable_dns_hostnames": true,
                "enable_dns_support": true,
                "tags": {
                  "Name": "main-vpc"
                }
              }
            }
          ],
          "child_modules": [
            {
              "address": "module.vpc.module.subnets",
              "resources": [
                {
                  "address": "module.vpc.module.subnets.aws_subnet.public[0]",
                  "mode": "managed",
                  "type": "aws_subnet",
                  "name": "public",
                  "index": 0,
                  "provider_name": "registry.terraform.io/hashicorp/aws",
                  "values": {
                    "cidr_block": "10.0.1.0/24",
                    "availability_zone": "us-east-1a"
                  }
                }
              ]
            }
          ]
        }
      ]
    }
  }
}
```

## Test Implementation Examples

### Unit Test Example
```rust
#[cfg(test)]
mod parser_tests {
    use super::*;
    use pretty_assertions::assert_eq;
    
    #[test]
    fn test_parse_create_action() {
        let input = r#"
        # aws_s3_bucket.example will be created
        + resource "aws_s3_bucket" "example" {
            + bucket = "my-bucket"
            + id     = (known after apply)
          }
        "#;
        
        let result = parse_resource_block(input).unwrap();
        
        assert_eq!(result.action, ActionType::Create);
        assert_eq!(result.resource_type, "aws_s3_bucket");
        assert_eq!(result.name, "example");
        assert_eq!(result.attributes.get("bucket").unwrap(), "my-bucket");
    }
    
    #[test]
    fn test_clean_ansi_codes() {
        let input = "\x1b[32m+ resource\x1b[0m";
        let expected = "+ resource";
        
        assert_eq!(clean_ansi_codes(input), expected);
    }
}
```

### Integration Test Example
```rust
#[cfg(test)]
mod integration_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_cli_with_file_input() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "Plan: 2 to add, 1 to change, 0 to destroy.").unwrap();
        
        let mut cmd = Command::cargo_bin("tfdiff").unwrap();
        cmd.arg(file.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("2 to add"))
            .stdout(predicate::str::contains("1 to change"));
    }
    
    #[tokio::test]
    async fn test_web_server_launch() {
        let mut cmd = Command::cargo_bin("tfdiff").unwrap();
        let output = cmd
            .arg("--web")
            .arg("--port")
            .arg("0") // Random port
            .timeout(std::time::Duration::from_secs(2))
            .output()
            .unwrap();
        
        assert!(output.status.success());
    }
}
```

### E2E Test Example
```rust
#[cfg(test)]
mod e2e_tests {
    use super::*;
    
    #[test]
    fn test_terraform_version_compatibility() {
        let versions = ["0.12", "0.13", "0.14", "0.15", "1.0", "1.5"];
        
        for version in versions {
            let fixture_path = format!("tests/fixtures/terraform/v{}/plan.json", version);
            let content = std::fs::read_to_string(&fixture_path).unwrap();
            
            let result = parse_terraform_json(&content);
            assert!(result.is_ok(), "Failed to parse Terraform {} output", version);
        }
    }
    
    #[test]
    fn test_large_plan_performance() {
        let large_plan = generate_large_plan(1000); // 1000 resources
        let start = std::time::Instant::now();
        
        let result = parse_terraform_output(&large_plan);
        
        let duration = start.elapsed();
        assert!(result.is_ok());
        assert!(duration.as_millis() < 100, "Parsing took too long: {:?}", duration);
    }
}
```

## Testing Best Practices

### 1. Test Organization
```rust
// Group related tests with modules
#[cfg(test)]
mod tests {
    mod parser {
        mod should {
            #[test]
            fn parse_simple_create_action() { }
            
            #[test]
            fn handle_nested_json_objects() { }
        }
        
        mod should_not {
            #[test]
            fn panic_on_malformed_input() { }
        }
    }
}
```

### 2. Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_parser_doesnt_crash(input in any::<String>()) {
        // Parser should never panic, only return errors
        let _ = parse_terraform_output(&input);
    }
    
    #[test]
    fn test_clean_function_is_idempotent(input in any::<String>()) {
        let cleaned_once = clean_input(&input);
        let cleaned_twice = clean_input(&cleaned_once);
        assert_eq!(cleaned_once, cleaned_twice);
    }
}
```

### 3. Snapshot Testing
```rust
use insta::assert_snapshot;

#[test]
fn test_terminal_output_formatting() {
    let plan = load_fixture("simple_plan.json");
    let output = format_terminal_output(&plan);
    
    assert_snapshot!(output);
}
```

### 4. Mocking External Dependencies
```rust
use mockall::automock;

#[automock]
trait FileSystem {
    fn read_file(&self, path: &Path) -> io::Result<String>;
    fn watch_file(&self, path: &Path) -> Result<Receiver<Event>>;
}

#[test]
fn test_watch_mode() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_read_file()
        .returning(|_| Ok("Plan: 1 to add".to_string()));
    
    // Test implementation
}
```

## Performance Testing

### Benchmark Suite
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parser_benchmark(c: &mut Criterion) {
    let small_plan = load_fixture("small_plan.txt");
    let medium_plan = load_fixture("medium_plan.txt");
    let large_plan = load_fixture("large_plan.txt");
    
    let mut group = c.benchmark_group("parser");
    
    group.bench_function("small_plan", |b| {
        b.iter(|| parse_terraform_output(black_box(&small_plan)))
    });
    
    group.bench_function("medium_plan", |b| {
        b.iter(|| parse_terraform_output(black_box(&medium_plan)))
    });
    
    group.bench_function("large_plan", |b| {
        b.iter(|| parse_terraform_output(black_box(&large_plan)))
    });
    
    group.finish();
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);
```

## CI/CD Testing Configuration

### GitHub Actions Workflow
```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: cargo test --all-features --verbose
    
    - name: Run tests with coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Xml --all-features
    
    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        file: ./cobertura.xml
    
    - name: Run clippy
      run: cargo clippy --all-features -- -D warnings
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run benchmarks
      run: cargo bench --no-run
```

## Test Data Generation

### Fixture Generator Script
```rust
// scripts/generate_fixtures.rs
use serde_json::json;
use std::fs;

fn generate_large_plan(resource_count: usize) -> serde_json::Value {
    let mut resources = Vec::new();
    
    for i in 0..resource_count {
        resources.push(json!({
            "address": format!("aws_instance.server_{}", i),
            "mode": "managed",
            "type": "aws_instance",
            "name": format!("server_{}", i),
            "change": {
                "actions": ["create"],
                "after": {
                    "instance_type": "t3.micro",
                    "ami": "ami-12345678"
                }
            }
        }));
    }
    
    json!({
        "format_version": "1.2",
        "resource_changes": resources
    })
}

fn main() {
    // Generate various sized plans
    let sizes = vec![10, 100, 1000, 10000];
    
    for size in sizes {
        let plan = generate_large_plan(size);
        let filename = format!("tests/fixtures/generated/plan_{}_resources.json", size);
        fs::write(filename, serde_json::to_string_pretty(&plan).unwrap()).unwrap();
    }
}
```

## Mutation Testing

```toml
# cargo-mutants configuration
[mutants]
exclude_globs = ["tests/**", "benches/**"]
exclude_re = ["println!", "eprintln!", "debug!"]
```

## Fuzz Testing

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = tfdiff::parser::parse_terraform_output(s);
    }
});
```

## Documentation Testing

```rust
/// Parses a Terraform plan output and returns a structured representation.
/// 
/// # Examples
/// 
/// ```
/// use tfdiff::parser::parse_terraform_output;
/// 
/// let input = "Plan: 1 to add, 0 to change, 0 to destroy.";
/// let result = parse_terraform_output(input).unwrap();
/// 
/// assert_eq!(result.summary.add, 1);
/// assert_eq!(result.summary.change, 0);
/// assert_eq!(result.summary.destroy, 0);
/// ```
/// 
/// # Errors
/// 
/// Returns an error if the input cannot be parsed as valid Terraform output.
pub fn parse_terraform_output(input: &str) -> Result<TerraformPlan> {
    // Implementation
}
```

## Test Helpers and Utilities

```rust
// tests/common/mod.rs
pub mod fixtures {
    use std::path::PathBuf;
    
    pub fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }
    
    pub fn load_fixture(name: &str) -> String {
        std::fs::read_to_string(fixture_path(name))
            .expect("Failed to load fixture")
    }
}

pub mod assertions {
    use crate::models::*;
    
    pub fn assert_resource_count(plan: &TerraformPlan, expected: usize) {
        assert_eq!(
            plan.resources.len(),
            expected,
            "Expected {} resources, found {}",
            expected,
            plan.resources.len()
        );
    }
}
```

This comprehensive testing strategy ensures high code quality, reliability, and maintainability for the Terraform Diff CLI tool.