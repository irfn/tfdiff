#!/bin/bash

# Script to check project structure and identify potential issues

echo "Checking project structure..."

# Check if all required directories exist
required_dirs=("src" "src/parser" "src/formatter" "src/models" "src/ui" "tests" "tests/fixtures" "benches")
for dir in "${required_dirs[@]}"; do
    if [ -d "$dir" ]; then
        echo "✓ Directory exists: $dir"
    else
        echo "✗ Missing directory: $dir"
    fi
done

# Check if all main source files exist
required_files=(
    "Cargo.toml"
    "src/main.rs"
    "src/lib.rs"
    "src/models/mod.rs"
    "src/parser/mod.rs"
    "src/parser/cleaner.rs"
    "src/parser/terraform.rs"
    "src/parser/diff.rs"
    "src/formatter/mod.rs"
    "src/formatter/terminal.rs"
    "src/formatter/json.rs"
    "src/formatter/html.rs"
    "src/formatter/markdown.rs"
    "src/ui/mod.rs"
    "src/ui/terminal.rs"
    "src/ui/web.rs"
)

echo ""
echo "Checking source files..."
for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✓ File exists: $file"
    else
        echo "✗ Missing file: $file"
    fi
done

# Check for compilation issues (basic syntax check)
echo ""
echo "Checking for basic syntax issues..."
for rs_file in $(find src -name "*.rs" -type f); do
    # Check for unmatched braces
    open_braces=$(grep -o '{' "$rs_file" | wc -l)
    close_braces=$(grep -o '}' "$rs_file" | wc -l)
    if [ $open_braces -ne $close_braces ]; then
        echo "⚠ Potential brace mismatch in $rs_file ({: $open_braces, }: $close_braces)"
    fi
    
    # Check for TODO comments
    if grep -q "TODO" "$rs_file"; then
        echo "ℹ TODO found in $rs_file"
    fi
done

echo ""
echo "Checking test structure..."
# Check test files
test_files=(
    "tests/lib.rs"
    "tests/common/mod.rs"
    "tests/unit/mod.rs"
    "tests/integration/mod.rs"
)

for file in "${test_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✓ Test file exists: $file"
    else
        echo "✗ Missing test file: $file"
    fi
done

echo ""
echo "Project structure check complete!"