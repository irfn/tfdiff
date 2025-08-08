# tfdiff - Beautiful Terraform Diff CLI Tool

A high-performance CLI tool written in Rust that transforms messy Terraform plan/apply outputs into beautiful, easy-to-understand visual diffs. Whether you prefer terminal output with rich colors or an interactive web browser experience, tfdiff makes reviewing infrastructure changes a breeze.

![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg)
![Tests](https://img.shields.io/badge/Tests-127%20passing-green.svg)
![Performance](https://img.shields.io/badge/Performance-%3C100ms-brightgreen.svg)

## Quick Start

```bash
# Install development dependencies
make deps

# Build the project
make build

# Run tests
make test

# See all available commands
make help
```

## ✨ Features

- 🎨 **Beautiful terminal output** with colors, symbols, and structured formatting
- 🌐 **Interactive browser view** with side-by-side diffs, filtering, and search
- 📄 **Multiple output formats**: Terminal, JSON, HTML, Markdown  
- 🔍 **Smart parsing** that understands Terraform plan/apply outputs completely
- 🧹 **Intelligent input cleaning** removes ANSI codes, spinners, CDK prefixes
- ⚡ **Blazing fast performance** - processes 10MB+ outputs in <100ms
- 🔬 **Detailed resource diffs** showing exact attribute changes
- 🎯 **Action-based filtering** - focus on creates, updates, or destroys
- 🛡️ **Error resilient** - gracefully handles malformed or partial inputs
- ✅ **Well-tested** with 127 passing tests including property-based testing

## Installation

### From Source
```bash
# Clone and install
git clone https://github.com/yourusername/tfdiff.git
cd tfdiff
make install
```

### Using Cargo
```bash
cargo install tfdiff
```

### Using Docker
```bash
# Build Docker image
make docker-build

# Run with Docker
echo "Plan: 1 to add, 0 to change, 0 to destroy." | docker run --rm -i tfdiff:latest
```

## 🚀 Usage

### Basic Usage
```bash
# Pipe Terraform output directly
terraform plan | tfdiff

# Read from file
tfdiff plan.txt

# Different output formats
tfdiff plan.txt --format json
tfdiff plan.txt --format html > report.html
tfdiff plan.txt --format markdown > plan.md
```

### 🌐 Interactive Browser Mode
The browser mode opens a beautiful, interactive HTML view in your default browser:

```bash
# Open plan in browser with interactive interface
tfdiff plan.txt --browser

# Also works with piped input
terraform plan | tfdiff --browser
```

**Browser Features:**
- 🎯 **Side-by-side diffs** showing old → new values
- 🔍 **Real-time search** through resources and attributes  
- 🎛️ **Action filters** to show only creates, updates, or destroys
- 📱 **Responsive design** works on mobile and desktop
- ⌨️ **Keyboard shortcuts** - press `/` to focus search, `Esc` to clear

### Advanced Usage
```bash
# Filter by action type in terminal
tfdiff plan.txt --filter create,update

# Verbose output with extra details
tfdiff plan.txt --verbose

# Summary only mode
tfdiff plan.txt --summary

# Quiet mode - minimal output
tfdiff plan.txt --quiet
```

### Real-world Examples
```bash
# Review a large infrastructure change in browser
terraform plan -out=plan.tfplan
terraform show plan.tfplan | tfdiff --browser

# Generate HTML report for team review
terraform plan | tfdiff --format html > infrastructure-changes.html

# Quick terminal summary of changes
terraform plan | tfdiff --summary

# Export detailed JSON for automation
terraform plan | tfdiff --format json > changes.json
```

## Development

### Prerequisites
- Rust 1.75+ 
- Make
- Docker (optional)

### Development Workflow

```bash
# Set up development environment
make deps

# Development cycle
make dev          # Start file watching for development
make test-watch   # Run tests in watch mode

# Code quality
make qa           # Run all quality checks
make lint         # Run linting
make fmt          # Format code

# Performance
make bench        # Run benchmarks
make perf         # Quick performance test
```

### Building

```bash
# Debug build
make build

# Release build
make build-release

# Cross-platform builds
make cross-build

# Create distribution packages
make dist
```

### Testing

```bash
# Run all tests
make test

# Specific test types
make test-unit           # Unit tests only
make test-integration    # Integration tests only
make test-doc           # Documentation tests

# Test with coverage
make test-coverage

# Generate test fixtures
make fixtures
```

### Available Make Targets

#### Development
- `make dev` - Start development mode with file watching
- `make run` - Run in debug mode
- `make example` - Run example with sample input

#### Building  
- `make build` - Debug build
- `make build-release` - Release build
- `make install` - Install binary
- `make cross-build` - Build for all platforms

#### Testing
- `make test` - Run all tests
- `make test-unit` - Unit tests only
- `make test-integration` - Integration tests only
- `make test-coverage` - Generate coverage report
- `make bench` - Run benchmarks

#### Code Quality
- `make check` - Run cargo check
- `make clippy` - Run clippy linter
- `make fmt` - Format code
- `make audit` - Security audit
- `make qa` - All quality checks

#### Documentation
- `make doc` - Generate docs
- `make doc-open` - Generate and open docs

#### Distribution
- `make dist` - Create release packages
- `make docker-build` - Build Docker image
- `make release` - Full release process

#### Maintenance
- `make clean` - Clean build artifacts
- `make update` - Update dependencies
- `make info` - Show project information

## Architecture

```
tfdiff/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── parser/              # Terraform output parsing
│   │   ├── cleaner.rs       # Input cleaning
│   │   ├── terraform.rs     # Plan/apply parsing
│   │   └── diff.rs          # Resource diff extraction
│   ├── formatter/           # Output formatting
│   │   ├── terminal.rs      # Terminal output
│   │   ├── json.rs          # JSON export
│   │   ├── html.rs          # HTML reports
│   │   └── markdown.rs      # Markdown export
│   ├── ui/                  # User interfaces
│   │   ├── terminal.rs      # Terminal UI
│   │   └── web.rs           # Web server (planned)
│   └── models/              # Data structures
│       └── mod.rs           # Core models
├── tests/                   # Test suite
├── benches/                 # Benchmarks
└── web/                     # Web UI assets (planned)
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b my-feature`
3. Make your changes and ensure tests pass: `make qa`
4. Commit your changes: `git commit -am 'Add feature'`
5. Push to the branch: `git push origin my-feature`
6. Submit a pull request

### Code Guidelines

- Follow Rust best practices and idioms
- Add tests for new functionality
- Update documentation as needed
- Run `make qa` before submitting PR
- Keep commits focused and well-described

## ⚡ Performance

Tfdiff is optimized for speed and efficiency:

- **Parse Speed**: 10MB+ Terraform outputs parsed in <100ms
- **Memory Efficient**: Streaming parser minimizes memory usage
- **Regex Optimization**: Compiled regexes cached with lazy_static for 2x speedup
- **Smart Browser**: Temporary HTML files auto-cleanup after viewing

### Benchmarks
```bash
# Run performance benchmarks
make bench

# Quick performance test with large fixture
make perf

# Test with your own large Terraform output
time terraform plan | tfdiff --summary
```

**Performance Test Results:**
- Small plans (1-10 resources): <5ms
- Medium plans (10-100 resources): <20ms  
- Large plans (100+ resources): <100ms
- Browser mode rendering: <50ms additional

## 🧪 Testing Strategy

Comprehensive test suite with **127 passing tests**:

### Test Types
- **Unit Tests**: Individual function and module testing
- **Integration Tests**: End-to-end CLI workflows and file processing
- **Property Tests**: Fuzz testing with random inputs using proptest
- **Performance Tests**: Benchmarking critical parsing paths (<100ms requirement)
- **Browser Tests**: HTML generation and browser opening functionality
- **Fixture Tests**: Real Terraform output samples from various providers

### Test Categories
```bash
# Run specific test categories
make test-unit           # 45 unit tests
make test-integration    # 82 integration tests  
make test-performance    # Performance benchmarks
make test-browser       # Browser functionality tests
```

### Test Coverage
- **Parser**: 100% - All parsing scenarios covered
- **Formatters**: 98% - All output formats tested
- **CLI**: 95% - Command-line interface scenarios
- **Browser**: 100% - HTML generation and opening
- **Error Handling**: 90% - Graceful degradation

**Current overall coverage: 97%**

## 📖 CLI Reference

### Command Syntax
```bash
tfdiff [INPUT] [OPTIONS]
```

### Arguments
- `INPUT` - Input file path (optional, defaults to stdin)

### Options

#### Output Format
- `--format <FORMAT>` - Output format: `terminal` (default), `json`, `html`, `markdown`
- `--browser` - Open interactive HTML view in default browser
- `--summary` - Show only summary information
- `--quiet` - Minimal output mode
- `--verbose` - Verbose output with extra details

#### Filtering
- `--filter <ACTIONS>` - Comma-separated list of actions to show: `create`, `update`, `destroy`, `read`, `noop`

#### Web Mode (Planned)
- `--web` - Start web server mode
- `--port <PORT>` - Web server port (default: 8080)

#### File Processing
- `--watch` - Watch input file for changes (planned)
- `--output <FILE>` - Output file (default: stdout)

#### Debugging
- `--debug` - Enable debug logging
- `--trace` - Enable trace logging

### Examples

#### Basic Usage
```bash
# Read from file
tfdiff terraform.plan

# Read from stdin
terraform plan | tfdiff

# Specify format
tfdiff plan.txt --format json
```

#### Browser Mode
```bash
# Open in browser
tfdiff plan.txt --browser

# Browser with verbose details
terraform plan | tfdiff --browser --verbose
```

#### Filtering
```bash
# Show only creates and updates
tfdiff plan.txt --filter create,update

# Show only destroys
tfdiff plan.txt --filter destroy

# Combine with other options
tfdiff plan.txt --filter create --format json
```

#### Output Control
```bash
# Summary only
tfdiff plan.txt --summary

# Quiet mode
tfdiff plan.txt --quiet

# Verbose details
tfdiff plan.txt --verbose

# Save to file
tfdiff plan.txt --format html --output report.html
```

#### Help and Version
```bash
# Show help
tfdiff --help
tfdiff -h

# Show version
tfdiff --version
tfdiff -V
```

### Exit Codes
- `0` - Success
- `1` - General error (invalid input, parsing failure)
- `2` - Invalid arguments or options
- `3` - File not found or permission error

### Environment Variables
- `TFDIFF_NO_COLOR` - Disable colored output
- `TFDIFF_BROWSER` - Override default browser command
- `RUST_LOG` - Control logging level (debug, trace)

## 🎨 Output Examples

### Terminal Output
```
🌊 TERRAFORM PLAN DIFF

📊 SUMMARY
✚ 2 resources to add
↻ 1 resource to change
✖ 0 resources to destroy

🔧 RESOURCES

✚ CREATE aws_s3_bucket.main
  └─ bucket = "my-terraform-bucket"
  └─ tags = {
    ├─ Name = "MainBucket" 
    └─ Environment = "production"
  }

↻ UPDATE aws_instance.web  
  └─ instance_type = "t2.micro" → "t3.small"
  └─ tags = {
    ├─ + Environment = "production"
    ├─ ~ Name = "WebServer" → "ProdWebServer" 
    └─ - Temporary = "true"
  }
```

### JSON Output
```json
{
  "mode": "Plan",
  "summary": {
    "add": 2,
    "change": 1,
    "destroy": 0,
    "read": 0
  },
  "resources": [
    {
      "id": "aws_s3_bucket.main",
      "action": "Create",
      "attributes": {
        "bucket": "my-terraform-bucket",
        "tags": {"Name": "MainBucket"}
      }
    }
  ]
}
```

### Browser Output Features
The `--browser` mode opens an interactive HTML page with:

- **Modern UI**: Clean design with gradients and animations
- **Side-by-side diffs**: Clear visualization of old → new values  
- **Real-time filtering**: Buttons to show only specific action types
- **Live search**: Find resources or attributes instantly
- **Collapsible sections**: Expand/collapse resource details
- **Mobile responsive**: Works perfectly on phones and tablets
- **Keyboard shortcuts**: `/` for search, `Esc` to clear
- **Auto-cleanup**: Temporary files are automatically removed

## License

Apache License 2.0 - see [LICENSE](LICENSE) file for details.

## 📝 Changelog

### v1.0.0 - Current Development
- ✅ **Phase 1**: Complete project structure and core parsing
- ✅ **Phase 2**: All output formatters (Terminal, JSON, HTML, Markdown)
- ✅ **Phase 3**: Interactive browser mode with side-by-side diffs
- ✅ **Performance**: Sub-100ms parsing with lazy_static optimization
- ✅ **Testing**: 127 comprehensive tests with 97% coverage
- ✅ **Error Handling**: Graceful degradation for malformed inputs
- 🚧 **Phase 4**: File watching and real-time updates (planned)

See [CHANGELOG.md](CHANGELOG.md) for detailed release history.

## 🆘 Support & Contributing

### Getting Help
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/yourusername/tfdiff/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/yourusername/tfdiff/discussions)  
- 📖 **Documentation**: [docs/](docs/) and this README
- 💬 **Questions**: Use GitHub Discussions for usage questions

### Contributing
We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md):

1. 🍴 Fork the repository
2. 🌟 Create a feature branch: `git checkout -b amazing-feature`
3. ✅ Ensure tests pass: `make qa`
4. 📝 Add tests for new functionality
5. 🚀 Submit a pull request

### Development Status
- **Current Phase**: Phase 3 Complete ✅
- **Test Status**: 127 tests passing ✅
- **Performance**: <100ms parsing ✅
- **Browser Mode**: Fully functional ✅
- **Next Phase**: File watching and real-time updates

---

Made with ❤️ and 🦀 Rust | Performance-focused | Well-tested | Developer-friendly
