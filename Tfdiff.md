# Terraform Diff CLI Tool - Comprehensive Development Prompt

## Project Overview
Build a high-performance CLI tool in Rust that parses Terraform plan/apply outputs and displays them in a beautiful, easy-to-understand format. The tool should work both as a pure terminal application and optionally launch a web UI for complex diffs.

## Core Requirements

### 1. CLI Features
- **Input Methods**:
  - Pipe input: `terraform plan | tfdiff`
  - File input: `tfdiff plan.txt`
  - Multiple files: `tfdiff plan1.txt plan2.txt --compare`
  - Watch mode: `tfdiff --watch plan.txt` (auto-refresh on file changes)

- **Output Formats**:
  - Terminal output with ANSI colors (default)
  - JSON output: `tfdiff --json`
  - HTML output: `tfdiff --html > report.html`
  - Markdown output: `tfdiff --markdown`
  - Web UI: `tfdiff --web` (launches browser)

- **Display Options**:
  - Summary only: `tfdiff --summary`
  - Filter by action: `tfdiff --filter create,update`
  - Filter by resource type: `tfdiff --resource-type aws_s3_bucket`
  - Verbose mode: `tfdiff -v` (show all attributes)
  - Quiet mode: `tfdiff -q` (only show resource names)

### 2. Parser Requirements
- **Clean Input Processing**:
  - Remove ANSI escape codes
  - Remove terminal control sequences (cursor movements)
  - Handle spinner characters (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
  - Strip CDK/CDKTF prefixes (e.g., "base14-cd")
  - Handle wrapped lines and multi-line values

- **Resource Detection**:
  - Plan format: `# resource.type.name will be created/updated/destroyed/read`
  - Apply format: `resource.type.name: Creating.../Modifying.../Destroying.../Reading...`
  - Data sources: `# data.type.name will be read during apply`
  - Handle nested resources and modules

- **Diff Parsing**:
  - Detect additions: lines starting with `+` or containing `[32m+[0m`
  - Detect removals: lines starting with `-` or containing `[31m-[0m`
  - Detect changes: lines starting with `~` or containing `[33m~[0m`
  - Parse JSON diffs for complex attributes
  - Handle `(known after apply)` values
  - Parse `# (X unchanged attributes hidden)` comments

- **Summary Extraction**:
  - Plan summary: `Plan: X to add, Y to change, Z to destroy`
  - Apply summary: `Apply complete! Resources: X added, Y changed, Z destroyed`
  - Parse warnings and errors
  - Extract timing information if available

### 3. Terminal UI Design
```
═══════════════════════ TERRAFORM PLAN DIFF ═══════════════════════

                    ┌────────────────────────────────┐
                    │ ✚ 2 to add  │  ↻ 3 to change │
                    │ ✖ 1 to destroy │ ⇐ 1 to read │
                    └────────────────────────────────┘

═══════════════════════════════════════════════════════════════════
✚ CREATE aws_ecr_repository.example
───────────────────────────────────────────────────────────────────
  + arn                  = (known after apply)
  + name                 = "example-repo"
  + image_tag_mutability = "MUTABLE"
  
  + image_scanning_configuration {
      + scan_on_push = true
    }

═══════════════════════════════════════════════════════════════════
↻ UPDATE aws_s3_bucket.example
───────────────────────────────────────────────────────────────────
OLD                              │ NEW
──────────────────────────────── │ ────────────────────────────────
- versioning = false             │ + versioning = true
- lifecycle_rule { ... }         │ + lifecycle_rule { ... }
```

### 4. Web UI Features
- **Server Mode**: 
  - Launch local web server on configurable port
  - Serve single-page application
  - WebSocket support for live updates in watch mode
  - REST API for fetching diff data

- **UI Components**:
  - Searchable resource list with filters
  - Side-by-side diff viewer with syntax highlighting
  - Collapsible resource sections
  - Export functionality (PDF, HTML, Markdown)
  - Dark/light theme toggle
  - Keyboard shortcuts for navigation

### 5. Technical Architecture

#### Project Structure
```
tfdiff/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point and argument parsing
│   ├── lib.rs               # Library root
│   ├── parser/
│   │   ├── mod.rs           # Parser module
│   │   ├── cleaner.rs       # Input cleaning and normalization
│   │   ├── terraform.rs     # Terraform-specific parsing
│   │   └── diff.rs          # Diff extraction and analysis
│   ├── formatter/
│   │   ├── mod.rs           # Formatter module
│   │   ├── terminal.rs      # Terminal output formatting
│   │   ├── json.rs          # JSON output
│   │   ├── html.rs          # HTML generation
│   │   └── markdown.rs      # Markdown generation
│   ├── ui/
│   │   ├── mod.rs           # UI module
│   │   ├── terminal.rs      # Terminal UI components
│   │   └── web.rs           # Web server and API
│   └── models/
│       └── mod.rs           # Data structures
├── web/
│   ├── index.html           # Web UI entry point
│   ├── app.js               # Frontend application
│   └── style.css            # Styles
└── tests/
    ├── fixtures/            # Test terraform outputs
    └── integration.rs       # Integration tests
```

#### Key Dependencies
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }      # CLI argument parsing
regex = "1.10"                                          # Pattern matching
colored = "2.1"                                         # Terminal colors
serde = { version = "1.0", features = ["derive"] }     # Serialization
serde_json = "1.0"                                      # JSON support
tokio = { version = "1", features = ["full"] }         # Async runtime
axum = "0.7"                                           # Web framework
tower = "0.4"                                          # HTTP middleware
tower-http = { version = "0.5", features = ["fs"] }    # Static file serving
notify = "6.0"                                         # File watching
indicatif = "0.17"                                     # Progress bars
comfy-table = "7.0"                                    # Table formatting
similar = "2.3"                                        # Text diffing
syntect = "5.0"                                        # Syntax highlighting
```

#### Core Data Structures
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformPlan {
    pub mode: PlanMode,
    pub summary: Summary,
    pub resources: Vec<Resource>,
    pub data_sources: Vec<DataSource>,
    pub warnings: Vec<Warning>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanMode {
    Plan,
    Apply,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub add: usize,
    pub change: usize,
    pub destroy: usize,
    pub read: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub type_name: String,
    pub provider: String,
    pub action: ActionType,
    pub changes: Vec<Change>,
    pub attributes: HashMap<String, Value>,
    pub applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Create,
    Update,
    Destroy,
    Read,
    NoOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub path: Vec<String>,
    pub before: Option<Value>,
    pub after: Option<Value>,
    pub sensitive: bool,
    pub computed: bool,
}
```

### 6. Advanced Features

#### Performance Optimizations
- Stream large files instead of loading into memory
- Parallel parsing for multiple resources
- Lazy loading for web UI resource details
- Caching parsed results for watch mode
- Binary serialization for faster reloads

#### Error Handling
- Graceful handling of malformed Terraform output
- Helpful error messages with suggestions
- Partial parsing (show what can be parsed even if some parts fail)
- Debug mode with detailed parsing logs

#### Configuration
- Config file support (`~/.tfdiff.toml`)
- Customizable colors and symbols
- Resource type aliases
- Custom filters and views
- Export templates

#### Integration Features
- Git diff integration: `git diff | tfdiff --from-git`
- CI/CD mode with exit codes based on changes
- Slack/Discord webhook notifications
- GitHub PR comment formatting
- Terraform Cloud/Enterprise API integration

### 7. CLI Examples

```bash
# Basic usage
terraform plan | tfdiff
terraform apply | tfdiff

# File input
tfdiff plan.txt
tfdiff apply.log

# Filtering
tfdiff plan.txt --filter create,destroy
tfdiff plan.txt --resource-type aws_s3_bucket,aws_iam_role
tfdiff plan.txt --exclude-unchanged

# Output formats
tfdiff plan.txt --json | jq '.resources[] | select(.action == "create")'
tfdiff plan.txt --html > report.html
tfdiff plan.txt --markdown > plan.md

# Web UI
tfdiff plan.txt --web --port 8080
tfdiff --web --watch terraform.tfplan

# Comparison
tfdiff compare plan1.txt plan2.txt
tfdiff diff before.json after.json

# CI/CD integration
tfdiff plan.txt --ci --fail-on destroy
tfdiff plan.txt --junit-xml > test-results.xml
```

### 8. Testing Requirements

#### Unit Tests
- Parser tests for various Terraform output formats
- Cleaner tests for ANSI code removal
- Formatter tests for each output format
- Model serialization/deserialization tests

#### Integration Tests
- End-to-end CLI tests
- Web server API tests
- File watching tests
- Large file handling tests

#### Test Fixtures
- Terraform 0.12+ plan outputs
- Terraform 0.12+ apply outputs
- CDK for Terraform outputs
- OpenTofu outputs
- Terragrunt outputs
- Various error scenarios

### 9. Documentation Requirements

#### User Documentation
- Installation guide (cargo install, pre-built binaries, Docker)
- Quick start guide with common examples
- Configuration reference
- Output format specifications
- Troubleshooting guide

#### Developer Documentation
- Architecture overview
- Contributing guidelines
- Parser specification
- Plugin development guide
- API reference for web mode

### 10. Distribution

#### Release Artifacts
- Pre-built binaries for major platforms (Linux, macOS, Windows)
- Docker image
- Homebrew formula
- AUR package
- Debian/RPM packages
- Shell completion scripts (bash, zsh, fish, PowerShell)

#### Version Management
- Semantic versioning
- Changelog maintenance
- Backwards compatibility for major Terraform versions
- Migration guides for breaking changes

## Success Criteria

1. **Performance**: Process 10MB+ Terraform outputs in under 100ms
2. **Accuracy**: 100% accurate parsing of Terraform 0.12+ outputs
3. **Usability**: Intuitive CLI with helpful error messages
4. **Compatibility**: Support for Terraform, OpenTofu, CDK for Terraform
5. **Extensibility**: Plugin system for custom formatters and parsers

## Implementation Phases

### Phase 1: Core Parser and Terminal Output (Week 1-2)
- Basic CLI structure with clap
- Input cleaning and normalization
- Resource parsing and diff extraction
- Terminal formatter with colors

### Phase 2: Multiple Output Formats (Week 3)
- JSON formatter
- HTML generator
- Markdown formatter
- Summary and filtering options

### Phase 3: Web UI (Week 4-5)
- Web server setup with axum
- Frontend application
- REST API
- WebSocket for live updates

### Phase 4: Advanced Features (Week 6-7)
- File watching
- Comparison mode
- CI/CD integrations
- Configuration system

### Phase 5: Polish and Release (Week 8)
- Comprehensive testing
- Documentation
- Distribution setup
- Performance optimization

## Example Implementation Snippets

### CLI Argument Structure
```rust
#[derive(Parser)]
#[command(name = "tfdiff")]
#[command(about = "Beautiful Terraform plan and apply output formatter")]
struct Cli {
    /// Input file (reads from stdin if not provided)
    input: Option<PathBuf>,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "terminal")]
    format: OutputFormat,
    
    /// Filter by action type
    #[arg(short, long, value_delimiter = ',')]
    filter: Vec<ActionType>,
    
    /// Launch web UI
    #[arg(short, long)]
    web: bool,
    
    /// Port for web UI
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// Watch file for changes
    #[arg(short, long)]
    watch: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}
```

### Parser Pipeline
```rust
pub fn parse_terraform_output(input: &str) -> Result<TerraformPlan> {
    let cleaned = clean_input(input)?;
    let lines = cleaned.lines().collect::<Vec<_>>();
    
    let mode = detect_mode(&lines)?;
    let summary = extract_summary(&lines)?;
    let resources = parse_resources(&lines)?;
    let warnings = extract_warnings(&lines)?;
    
    Ok(TerraformPlan {
        mode,
        summary,
        resources,
        warnings,
        ..Default::default()
    })
}
```

This comprehensive prompt should provide clear direction for building a professional-grade Terraform diff tool in Rust with both CLI and web UI capabilities.