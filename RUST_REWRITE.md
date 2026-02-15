# Markdownlint Rust Rewrite Guide

## Prerequisites

Before starting, you'll need to:

1. **Fork the repository on GitHub:**
   - Go to https://github.com/DavidAnson/markdownlint
   - Click the "Fork" button in the top-right corner
   - This will create a copy under your GitHub account

2. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/markdownlint
   cd markdownlint
   git checkout -b rust-rewrite
   ```

## Project Structure

The Rust rewrite will be organized as follows:

```
markdownlint-rs/
├── Cargo.toml                 # Main workspace manifest
├── README.md                  # Rust-specific documentation
├── src/
│   ├── lib.rs                # Library root
│   ├── main.rs               # CLI binary (optional)
│   ├── rules/                # Rule implementations
│   │   ├── mod.rs            # Rule registry
│   │   ├── md001.rs          # Heading increment
│   │   ├── md003.rs          # Heading style
│   │   └── ...               # All 54 rules
│   ├── parser/               # Markdown parsing
│   │   ├── mod.rs
│   │   ├── micromark.rs      # Micromark integration
│   │   └── tokens.rs         # Token types
│   ├── config/               # Configuration handling
│   │   ├── mod.rs
│   │   ├── parse.rs          # Config file parsing
│   │   └── validation.rs     # Config validation
│   ├── lint/                 # Core linting engine
│   │   ├── mod.rs
│   │   ├── engine.rs         # Main linting logic
│   │   ├── cache.rs          # Performance caching
│   │   └── fixes.rs          # Auto-fix application
│   ├── types/                # Core type definitions
│   │   ├── mod.rs
│   │   ├── rule.rs           # Rule trait and types
│   │   ├── error.rs          # Error types
│   │   └── results.rs        # Lint results
│   └── helpers/              # Utility functions
│       ├── mod.rs
│       ├── text.rs           # String utilities
│       └── validators.rs     # Validation helpers
├── tests/                    # Integration tests
│   ├── test_fixtures/        # Test markdown files
│   └── integration_tests.rs
└── benches/                  # Performance benchmarks
    └── lint_benchmark.rs
```

## Core Dependencies (Cargo.toml)

Based on the analysis, here are the Rust equivalents:

### Parser Dependencies
- **markdown** or **pulldown-cmark** - Rust markdown parser (alternative to micromark)
- **comrak** - CommonMark parser with GFM extensions
- **serde** + **serde_json** - Serialization (config parsing)
- **serde_yaml** - YAML config support
- **toml** - TOML config support

### Async/Performance
- **tokio** - Async runtime (for async API)
- **rayon** - Data parallelism (concurrent file processing)
- **dashmap** - Concurrent hashmap (for caching)

### Utilities
- **regex** - Regular expressions
- **anyhow** / **thiserror** - Error handling
- **once_cell** - Lazy static initialization
- **unicode-width** - Character width calculations (like string-width)

### CLI (if building CLI tool)
- **clap** - Command-line argument parsing
- **colored** - Colored terminal output

## Implementation Phases

### Phase 1: Foundation (Week 1-2)
- [ ] Set up Cargo workspace
- [ ] Define core type system (Rule trait, LintError, LintResults)
- [ ] Implement configuration parsing
- [ ] Create token tree abstraction
- [ ] Build caching layer

### Phase 2: Parser Integration (Week 2-3)
- [ ] Integrate pulldown-cmark or comrak
- [ ] Build token tree from parser events
- [ ] Implement position tracking
- [ ] Add front matter detection
- [ ] Handle inline HTML

### Phase 3: Rule System (Week 3-6)
- [ ] Create Rule trait and registry
- [ ] Implement rule execution engine
- [ ] Port 10-15 high-priority rules (MD001, MD003, MD013, etc.)
- [ ] Add inline configuration support (disable/enable comments)
- [ ] Implement per-line configuration state

### Phase 4: Remaining Rules (Week 6-10)
- [ ] Port remaining ~40 rules
- [ ] Ensure test coverage for each rule
- [ ] Validate against original test fixtures

### Phase 5: Auto-fix System (Week 10-11)
- [ ] Implement fix application logic
- [ ] Add conflict resolution
- [ ] Test fix generation for all fixable rules

### Phase 6: API and Polish (Week 11-12)
- [ ] Create sync API
- [ ] Create async API
- [ ] Add custom rule support
- [ ] Performance optimization
- [ ] Documentation
- [ ] Examples

## Key Design Decisions

### 1. Parser Choice
**Recommendation: comrak**
- Full CommonMark + GFM support
- Event-based parsing similar to micromark
- Position information included
- Active maintenance

**Alternative: pulldown-cmark**
- Faster but less feature-complete
- May need extensions for GFM

### 2. Rule Trait Design
```rust
pub trait Rule: Send + Sync {
    fn names(&self) -> &[&'static str];
    fn description(&self) -> &'static str;
    fn tags(&self) -> &[&'static str];
    fn parser_type(&self) -> ParserType;

    fn lint(&self, params: &RuleParams) -> Vec<LintError>;

    // Optional async support
    fn lint_async(&self, params: &RuleParams) -> BoxFuture<'_, Vec<LintError>> {
        Box::pin(async move { self.lint(params) })
    }
}
```

### 3. Error Handling
Use `anyhow::Result` for fallible operations and `thiserror` for custom error types:
```rust
#[derive(Debug, thiserror::Error)]
pub enum MarkdownlintError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Rule error: {0}")]
    RuleError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### 4. Performance Optimizations
- Use `rayon` for parallel file processing
- Token caching with `DashMap` for thread-safe access
- Lazy rule initialization with `once_cell::Lazy`
- String interning for repeated strings (rule names, etc.)

### 5. API Compatibility
Maintain similar API surface:
```rust
// Sync API
pub fn lint_sync(options: &LintOptions) -> Result<LintResults>;

// Async API
pub async fn lint_async(options: &LintOptions) -> Result<LintResults>;

// Config API
pub fn read_config_sync(path: &Path) -> Result<Config>;
pub async fn read_config_async(path: &Path) -> Result<Config>;
```

## Migration Strategy

### Testing Approach
1. **Test Fixture Reuse:** Use existing test-repos and test files
2. **Snapshot Testing:** Compare output with original implementation
3. **Property Testing:** Use `proptest` for rule validation
4. **Benchmarking:** Compare performance with Node.js version

### Compatibility Goals
- [ ] 100% rule compatibility (same errors detected)
- [ ] Configuration format compatibility (JSON/YAML/TOML)
- [ ] Fix application compatibility (same fixes generated)
- [ ] API compatibility (similar function signatures)

### Performance Targets
- 5-10x faster than Node.js version
- <50ms startup time (vs ~200ms for Node.js)
- Linear scaling with file count (parallel processing)

## Example Code Snippets

### Rule Implementation Template
```rust
use crate::types::{Rule, RuleParams, LintError, ParserType};

pub struct MD001;

impl Rule for MD001 {
    fn names(&self) -> &[&'static str] {
        &["MD001", "heading-increment"]
    }

    fn description(&self) -> &'static str {
        "Heading levels should only increment by one level at a time"
    }

    fn tags(&self) -> &[&'static str] {
        &["headings"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::Micromark
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let mut errors = Vec::new();
        let mut prev_level = 0;

        for token in params.tokens.filter_by_type("atxHeading") {
            let level = get_heading_level(token);

            if prev_level > 0 && level > prev_level + 1 {
                errors.push(LintError {
                    line_number: token.start_line,
                    rule_names: self.names().to_vec(),
                    rule_description: self.description().to_string(),
                    error_detail: Some(format!(
                        "Expected: h{}; Actual: h{}",
                        prev_level + 1,
                        level
                    )),
                    ..Default::default()
                });
            }

            prev_level = level;
        }

        errors
    }
}
```

### Configuration Parsing
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuleConfig {
    Enabled(bool),
    Severity(String),
    Options(HashMap<String, serde_json::Value>),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default: Option<bool>,

    #[serde(flatten)]
    pub rules: HashMap<String, RuleConfig>,
}
```

## Next Steps

1. **Fork and clone the repository**
2. **Install Rust toolchain**
3. **Create `markdownlint-rs/` directory in the repo**
4. **Copy the Cargo.toml (next file)**
5. **Set up initial module structure**
6. **Begin Phase 1 implementation**

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Comrak Documentation](https://docs.rs/comrak/)
- [Pulldown-cmark Documentation](https://docs.rs/pulldown-cmark/)
- [Original Markdownlint](https://github.com/DavidAnson/markdownlint)
- [Micromark Specification](https://github.com/micromark/micromark)

## Questions and Considerations

1. **Should we maintain JavaScript API compatibility?**
   - Could provide Node.js bindings via neon or napi-rs
   - Allows gradual migration

2. **CLI tool included?**
   - Could replace markdownlint-cli
   - Single binary distribution
   - Faster startup and execution

3. **WASM target?**
   - Browser usage
   - Same as current demo
   - Smaller bundle size

4. **Version compatibility**
   - Start with v1.0.0 for Rust version?
   - Or match current version (0.40.0)?

---

**Created:** 2026-02-15
**Status:** Planning Phase
**Next Action:** Create Cargo.toml and initial project structure
