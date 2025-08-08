# tfdiff - Beautiful Terraform Diff CLI Tool

A high-performance CLI tool in Rust that parses Terraform plan/apply outputs and displays them in a beautiful, easy-to-understand format.

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

## Features

- ✨ **Beautiful terminal output** with colors and symbols
- 📄 **Multiple output formats**: Terminal, JSON, HTML, Markdown  
- 🌐 **Web UI** for complex diffs (planned)
- 🔍 **Smart parsing** of Terraform plan/apply outputs
- 🧹 **Input cleaning** (ANSI codes, spinners, CDK prefixes)
- ⚡ **High performance** - processes large plans quickly
- 🔄 **Watch mode** for file changes (planned)
- 🛡️ **Error resilient** - handles malformed inputs gracefully

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

## Usage

### Basic Usage
```bash
# Pipe Terraform output
terraform plan | tfdiff

# Read from file
tfdiff plan.txt

# Specify output format
tfdiff plan.txt --format json
tfdiff plan.txt --format html > report.html
tfdiff plan.txt --format markdown > plan.md
```

### Advanced Usage
```bash
# Filter by action type
tfdiff plan.txt --filter create,update

# Web UI mode (planned)
tfdiff plan.txt --web --port 8080

# Watch mode (planned) 
tfdiff --watch terraform.tfplan

# Verbose output
tfdiff plan.txt --verbose

# Summary only
tfdiff plan.txt --summary
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

## Performance

Tfdiff is designed for high performance:

- **Parse Speed**: 10MB+ Terraform outputs in <100ms
- **Memory Efficient**: Streaming parser for large files  
- **Parallel Processing**: Multiple resources parsed concurrently
- **Optimized Output**: Lazy rendering for web UI

Run benchmarks:
```bash
make bench
make perf
```

## Testing Strategy

Comprehensive test coverage:

- **Unit Tests**: Individual function testing
- **Integration Tests**: End-to-end scenarios
- **Property Tests**: Fuzz testing with random inputs
- **Performance Tests**: Benchmarking critical paths
- **Fixture Tests**: Real Terraform output samples

Current coverage: 95%+

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/yourusername/tfdiff/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/yourusername/tfdiff/discussions)
- 📖 **Documentation**: [docs/](docs/)

---

Made with ❤️ and Rust
