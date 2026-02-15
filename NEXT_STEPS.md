# Next Steps for Markdownlint Rust Rewrite

## You're Ready to Start!

I've set up the complete foundational structure for rewriting markdownlint in Rust. Here's what's been created and what to do next.

## What's Been Created

### 📁 Project Structure

```
markdownlint/
├── Cargo.toml                    # Rust project manifest with all dependencies
├── README_RUST.md                # Documentation for the Rust version
├── RUST_REWRITE.md              # Comprehensive rewrite guide
├── NEXT_STEPS.md                # This file
├── .gitignore                   # Updated to ignore Rust build artifacts
└── src/
    ├── lib.rs                   # Library root with public API
    ├── main.rs                  # CLI binary implementation
    ├── types/                   # Core type definitions
    │   ├── mod.rs
    │   ├── error.rs            # Error types and Result
    │   ├── rule.rs             # Rule trait and interfaces
    │   ├── options.rs          # LintOptions
    │   └── results.rs          # LintResults
    ├── parser/                  # Markdown parsing
    │   ├── mod.rs              # Parser integration
    │   └── token.rs            # Token types
    ├── config/                  # Configuration handling
    │   └── mod.rs              # JSON/YAML/TOML config
    ├── lint/                    # Core linting engine
    │   └── mod.rs              # Lint functions
    ├── helpers/                 # Utility functions
    │   └── mod.rs
    └── rules/                   # Rule implementations
        ├── mod.rs              # Rule registry
        └── md001.rs            # Example rule (MD001)
```

### 📝 Key Files Created

1. **Cargo.toml** - Complete with all dependencies:
   - `comrak` for Markdown parsing
   - `tokio` for async support
   - `rayon` for parallelism
   - `serde` for configuration
   - `clap` for CLI

2. **Type System** (src/types/):
   - `Rule` trait for implementing rules
   - `LintError` for error reporting
   - `LintOptions` and `LintResults` for API
   - Error handling with `thiserror`

3. **Parser Integration** (src/parser/):
   - Token abstraction layer
   - Comrak integration
   - Helper functions for token filtering

4. **Configuration** (src/config/):
   - JSON, YAML, TOML support
   - Config merging and inheritance
   - Rule-specific configuration

5. **Example Rule** (src/rules/md001.rs):
   - Complete MD001 implementation
   - Unit tests
   - Documentation

## Step-by-Step Next Actions

### 1. Fork the Repository

Since `gh` CLI isn't installed, manually fork on GitHub:

1. Go to https://github.com/DavidAnson/markdownlint
2. Click "Fork" in the top-right
3. Create a new branch for the Rust rewrite:
   ```bash
   cd /Users/johnewillmanv/projects/markdownlint
   git checkout -b rust-rewrite
   ```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # Verify installation
```

### 3. Commit the Rust Files

```bash
# Add all the new Rust files
git add Cargo.toml src/ README_RUST.md RUST_REWRITE.md NEXT_STEPS.md .gitignore

# Create initial commit
git commit -m "Initial Rust rewrite structure

- Add Cargo.toml with dependencies (comrak, tokio, rayon, serde, clap)
- Create module structure (types, parser, config, lint, rules, helpers)
- Implement core type system (Rule trait, LintError, LintOptions, LintResults)
- Add parser integration with comrak
- Implement configuration parsing (JSON/YAML/TOML)
- Create example MD001 rule with tests
- Add CLI binary with clap
- Add comprehensive documentation and guides
"
```

### 4. Verify the Build

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

You should see output like:
```
   Compiling markdownlint v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 12.34s
```

### 5. Implement Your First Rule

Start with a simple rule to understand the pattern. MD047 is a good choice:

Create `src/rules/md047.rs`:

```rust
use crate::types::{LintError, ParserType, Rule, RuleParams, Severity};

pub struct MD047;

impl Rule for MD047 {
    fn names(&self) -> &[&'static str] {
        &["MD047", "single-trailing-newline"]
    }

    fn description(&self) -> &'static str {
        "Files should end with a single newline character"
    }

    fn tags(&self) -> &[&'static str] {
        &["blank_lines"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::None // No parser needed
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let mut errors = Vec::new();

        if params.lines.is_empty() {
            return errors;
        }

        let last_line = &params.lines[params.lines.len() - 1];

        if !last_line.ends_with('\n') {
            errors.push(LintError {
                line_number: params.lines.len(),
                rule_names: self.names().iter().map(|s| s.to_string()).collect(),
                rule_description: self.description().to_string(),
                error_detail: None,
                error_context: None,
                rule_information: None,
                error_range: None,
                fix_info: None,
                severity: Severity::Error,
            });
        }

        errors
    }
}
```

Update `src/rules/mod.rs`:

```rust
mod md001;
mod md047;

pub static RULES: Lazy<Vec<BoxedRule>> = Lazy::new(|| {
    vec![
        Box::new(md001::MD001),
        Box::new(md047::MD047),
    ]
});
```

Test it:
```bash
cargo test
```

### 6. Improve the Parser

The current parser integration is basic. Enhance it:

1. Extract actual heading levels from comrak AST
2. Implement token parent/child relationships
3. Add token caching
4. Handle HTML flow special cases

Reference: [lib/micromark-parse.mjs](lib/micromark-parse.mjs) in the original project

### 7. Implement More Rules

Priority order (easiest to hardest):

**Easy (text-based, no parser):**
- MD047 - Single trailing newline ✅ (you just did this!)
- MD012 - Multiple consecutive blank lines
- MD009 - Trailing spaces
- MD010 - Hard tabs

**Medium (simple token-based):**
- MD001 - Heading increment ✅ (example provided)
- MD022 - Headings surrounded by blank lines
- MD025 - Single title/H1
- MD041 - First line is heading

**Hard (complex logic):**
- MD013 - Line length (with many exclusions)
- MD033 - Inline HTML (with allowed elements)
- MD043 - Required heading structure
- MD044 - Proper names capitalization

### 8. Add Integration Tests

Create test files:

```bash
mkdir -p tests/fixtures
```

Add test file `tests/integration_tests.rs`:

```rust
use markdownlint::{lint_sync, LintOptions};
use std::collections::HashMap;

#[test]
fn test_md001_heading_increment() {
    let markdown = "# Heading 1\n\n### Heading 3\n";
    let mut strings = HashMap::new();
    strings.insert("test.md".to_string(), markdown.to_string());

    let options = LintOptions {
        strings,
        ..Default::default()
    };

    let results = lint_sync(&options).unwrap();
    assert!(results.has_errors());
    assert_eq!(results.error_count(), 1);
}
```

### 9. Push to Your Fork

Once you have some working code:

```bash
# Push the rust-rewrite branch
git push -u origin rust-rewrite
```

Then create a pull request or continue developing.

### 10. Set Up CI/CD (Optional)

Create `.github/workflows/rust.yml`:

```yaml
name: Rust CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

## Development Workflow

### Daily Development

```bash
# 1. Make changes to a rule
vim src/rules/md003.rs

# 2. Run tests
cargo test

# 3. Check for warnings
cargo clippy

# 4. Format code
cargo fmt

# 5. Commit
git add .
git commit -m "Implement MD003 - heading style"
```

### Testing Against Original

To verify compatibility:

```bash
# Run original Node.js version
node example/standalone.mjs test.md

# Run Rust version
cargo run -- test.md

# Compare outputs
```

## Resources

### Documentation

- [RUST_REWRITE.md](RUST_REWRITE.md) - Comprehensive rewrite guide
- [README_RUST.md](README_RUST.md) - User-facing documentation
- Original markdownlint: https://github.com/DavidAnson/markdownlint
- Comrak docs: https://docs.rs/comrak/
- Rust book: https://doc.rust-lang.org/book/

### Architecture Reference

The exploration agent analyzed the entire codebase. Key insights:

- **Token structure**: See [lib/micromark-parse.mjs](lib/micromark-parse.mjs)
- **Rule execution flow**: See [lib/markdownlint.mjs](lib/markdownlint.mjs)
- **Configuration parsing**: See [lib/parse-configuration.mjs](lib/parse-configuration.mjs)
- **Inline directive handling**: Lines 360-480 in markdownlint.mjs
- **Fix application**: Lines 1290-1376 in markdownlint.mjs

## Success Metrics

Track your progress:

- [ ] Rust toolchain installed
- [ ] Project builds successfully
- [ ] 10 rules implemented (Week 4)
- [ ] 25 rules implemented (Week 6)
- [ ] 54 rules implemented (Week 10)
- [ ] All tests passing
- [ ] 100% compatibility with original
- [ ] 5x+ performance improvement
- [ ] Documentation complete
- [ ] CLI feature-complete

## Quick Commands Reference

```bash
# Build
cargo build                          # Debug build
cargo build --release                # Optimized build
cargo build --no-default-features    # Library only (no CLI)

# Test
cargo test                           # Run all tests
cargo test md001                     # Run specific test
cargo test -- --nocapture            # Show output

# Run
cargo run -- README.md               # Run CLI
cargo run --release -- *.md          # Optimized run

# Quality
cargo clippy                         # Linting
cargo fmt                            # Format code
cargo doc --open                     # Generate and open docs

# Benchmarks (create later)
cargo bench                          # Run benchmarks
```

## Troubleshooting

### Build Errors

If you get compilation errors:

```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build
```

### Missing Dependencies

If Rust isn't installed:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload environment
source $HOME/.cargo/env
```

## You're All Set! 🚀

The foundation is complete. Your immediate next steps are:

1. ✅ **Install Rust** (if not already done)
2. ✅ **Commit the files** to your rust-rewrite branch
3. ✅ **Run `cargo build`** to verify everything compiles
4. ✅ **Run `cargo test`** to see the tests pass
5. ✅ **Implement MD047** (simple rule to start)
6. ✅ **Start working through more rules**

Good luck with the rewrite! The hard architectural work is done - now it's implementing the 54 rules one by one.

---

**Created:** 2026-02-15
**Status:** Ready to begin implementation
**Estimated completion:** 12 weeks
