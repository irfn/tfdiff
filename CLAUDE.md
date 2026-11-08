# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`tfdiff` is a Rust CLI that parses human-readable `terraform plan` / `apply` text output (including CDKTF-wrapped output) or `terraform show -json` plan output, and re-renders it as terminal, JSON, HTML, or Markdown. `--browser` writes the HTML to a temp file and opens it via `open` / `xdg-open` / `start`.

## Commands

```bash
make build            # cargo build
make test             # cargo test (all targets)
make ci-check         # what CI runs: cargo check + fmt --check + clippy -D warnings + test
make lint             # fmt-check + clippy (--all-targets --all-features -D warnings)
make fmt              # cargo fmt
make bench            # criterion benches (benches/parser_bench.rs)
cargo run -- tests/fixtures/terraform/simple_plan.txt --format html
```

Clippy runs with `-D warnings` in CI, so any warning fails the build. `cargo clippy --fix` fails inside the Claude Code sandbox (it binds a local TCP port for locking), so apply lint fixes by hand there.

### Test layout (important)

Everything under `tests/` compiles into a **single** integration-test binary named `lib` (`tests/lib.rs` declares `common`, `unit`, `integration` as modules). As a result:

- `make test-unit` (`cargo test --lib`) does **not** run the `tests/unit/**` tests. Only `src/` has `#[cfg(test)]` code, and the one test there is in `main.rs`.
- Run one test or group by substring of its module path:
  ```bash
  cargo test --test lib test_parse_terraform_output_basic
  cargo test --test lib unit::parser::diff_test
  cargo test --test lib -- --list        # list all test names
  ```
- New test files have to be registered in the parent `mod.rs` (e.g. `tests/unit/parser/mod.rs`), or they won't compile or run.
- Fixtures live in `tests/fixtures/{terraform,providers,edge_cases,malformed}/`. Load them with the helpers in `tests/common/mod.rs` (`load_terraform_fixture`, `load_edge_case_fixture`, …). The `assertions` module there has shared plan assertions.
- `tests/integration/property_tests.rs` uses proptest. Failing seeds are saved in `proptest-regressions/`.

## Architecture

Pipeline: **raw text → `parser::clean_input` → line-based parse → `TerraformPlan` model → formatter → stdout**. JSON input skips the cleaner and line parser and goes straight to the model.

- `src/parser/terraform.rs` — `parse_terraform_output` is the entry point. Input starting with `{` goes to `json_plan::parse_terraform_json`, except `terraform plan -json` JSON lines, which go to the text parser. For text input it runs the cleaner, gets the mode and summary from the `Plan:` / `Apply complete!` line, then hands off to `diff::parse_resource_changes`.
- `src/parser/cleaner.rs` — strips ANSI codes and spinner characters, re-joins resource header lines that were split across two lines, and removes CDKTF stack prefixes. The prefix and line-join logic is tuned to real-world output (it hardcodes the `base14-` stack prefix and `# aws_` headers). New input quirks usually get handled here.
- `src/parser/diff.rs` — the core of the tool. It's a single-pass state machine over lines:
  - `# <type>.<name> will be created|updated|destroyed` lines start a new `Resource` (the regexes are `lazy_static` at the top of the file).
  - `+/~/- resource "..." {` starts the resource body. Brace counting tracks when the body ends.
  - **Create/Destroy** resources fill `Resource.attributes` (a `HashMap<String, Value>`). **Update** resources fill `Resource.changes` (`Vec<Change>` with `before`/`after`). Formatters branch on this split, so a change to one side usually needs a change in every formatter.
  - Multi-line array diffs in updates are collected separately (`detect_multiline_array_start` → `flush_multiline_array_change`).
  - `parse_terraform_value` converts Terraform literal text (strings, numbers, bools, `(known after apply)`, `null`, …) into `serde_json::Value`.
- `src/parser/json_plan.rs` — converts `resource_changes` from `terraform show -json` into the same model.
  - `resolve` merges `after_unknown` into the values as `(known after apply)` and masks anything marked in `before_sensitive` / `after_sensitive` as `(sensitive value)`.
  - **The JSON contains sensitive values in plain text**, so every path that copies values out of it must go through the masking. `test_no_secrets_in_any_output_format` checks this.
  - Replacements (`["delete","create"]` / `["create","delete"]`) map to `ActionType::Replace`. The text parser doesn't produce `Replace` yet.
- `src/models/mod.rs` — serde-serializable data model (`TerraformPlan`, `Resource`, `Change`, `ActionType`, …). JSON output is just `serde_json` on this model, so field renames change the JSON output format.
- `src/formatter/` — one `format_*_output(&TerraformPlan)` function per format. `html.rs` is the largest. It's a single self-contained page (inline CSS/JS) with a side-by-side diff for updates (`format_update_diff`) and a single-column view for create/destroy.
- `src/main.rs` — clap CLI. Applies `--filter` (by action name) to a clone of the plan before formatting.
- `src/ui/web.rs` and the `--web` / `--watch` / `--port` flags are placeholders and not implemented, even though axum/tokio/notify are dependencies.

## Docs caveat

`README.md`, `Tfdiff.md` (the original spec prompt), `Testing.md`, and `BUILD_STATUS.md` are partly aspirational. They describe flags (`--output`, `--debug`, `--compare`, `--resource-type`), env vars (`TFDIFF_NO_COLOR`, `TFDIFF_BROWSER`), exit codes, and coverage numbers that aren't in the code. The `Cli` struct in `src/main.rs` is the source of truth for CLI behavior.
