# Agent Guidelines for rust_eureka

This document provides guidelines for AI agents and contributors working on the `rust_eureka` project - a Rust client library for Netflix Eureka service discovery.

## Project Overview

**rust_eureka** is a Rust implementation of a client for Netflix Eureka. As a library crate, code quality, API stability, and comprehensive testing are critical priorities.

## Core Development Principles

### 1. Code Quality Standards

#### Clippy Linting (Required)
All code MUST pass Clippy with the following standards:

```bash
# Run clippy with warnings treated as errors
cargo clippy --all-targets --all-features -- -D warnings

# For workspace-level checks
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

**Requirements:**
- Zero clippy warnings allowed
- Address all `clippy::` lints before committing
- Use `#[allow(clippy::...)]` only with clear justification documented in comments
- Focus on these critical lints:
  - `clippy::unwrap_used` - Prefer proper error handling
  - `clippy::expect_used` - Use in tests only
  - `clippy::panic` - Avoid in library code
  - `clippy::missing_errors_doc` - Document error conditions
  - `clippy::missing_panics_doc` - Document panic conditions

#### Code Formatting (Required)
All code MUST be formatted with rustfmt:

```bash
# Format all code
cargo fmt --all

# Check formatting without modifying files
cargo fmt --all -- --check
```

**Requirements:**
- Use the default rustfmt configuration unless project has a `rustfmt.toml`
- Run `cargo fmt` before every commit
- CI must enforce formatting checks
- No formatting-related diffs should appear in PRs

### 2. Testing Requirements

#### Test Coverage Standards
As a service discovery client library, testing is critical for reliability:

```bash
# Run all tests
cargo test --all-features

# Run tests with output
cargo test --all-features -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --all-features --out Html --output-dir coverage
```

**Requirements:**
- **Minimum 80% code coverage** for all new code
- **100% coverage** for public API functions
- All public functions MUST have unit tests
- Integration tests in `tests/` directory for client workflows
- Test error paths, not just happy paths
- Use `test-logger` (already in dev-dependencies) for debugging

#### Test Organization
```rust
// Unit tests in module files
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        // Act
        // Assert
    }

    #[test]
    #[should_panic(expected = "specific error message")]
    fn test_error_condition() {
        // Test panic scenarios
    }
}

// Integration tests in tests/ directory
// tests/integration_test.rs
```

#### Async Testing
Since this project uses `tokio` and `futures`, ensure async tests are properly configured:

```rust
#[tokio::test]
async fn test_async_function() {
    // Async test code
}
```

### 3. Documentation Standards

#### Public API Documentation
All public items MUST have documentation:

```bash
# Check for missing documentation
cargo rustdoc -- -D missing_docs

# Generate and open documentation
cargo doc --no-deps --open
```

**Requirements:**
- Every public module, struct, enum, trait, and function needs doc comments
- Use `///` for item documentation
- Use `//!` for module-level documentation
- Include examples in doc comments when practical
- Document all error conditions with `# Errors` section
- Document all panic conditions with `# Panics` section

**Example:**
```rust
/// Registers an application instance with the Eureka server.
///
/// This function sends a registration request to the Eureka server
/// and returns a handle to the registered instance.
///
/// # Arguments
///
/// * `instance` - The instance information to register
///
/// # Errors
///
/// Returns an error if:
/// - The Eureka server is unreachable
/// - The instance data is invalid
/// - The server returns a non-2xx status code
///
/// # Examples
///
/// ```no_run
/// use rust_eureka::EurekaClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = EurekaClient::new("http://localhost:8761")?;
/// client.register(instance).await?;
/// # Ok(())
/// # }
/// ```
pub async fn register(&self, instance: Instance) -> Result<()> {
    // Implementation
}
```

### 4. Error Handling

#### Error Type Standards
- Use custom error types (already in `src/errors.rs`)
- Implement `std::error::Error` for all error types
- Use `thiserror` or similar for error derive macros (consider adding)
- Never use `unwrap()` or `expect()` in library code (tests are OK)
- Prefer `?` operator for error propagation

**Example:**
```rust
// Good
pub async fn fetch_instances(&self) -> Result<Vec<Instance>> {
    let response = self.client.get(url).await?;
    let instances = response.json().await?;
    Ok(instances)
}

// Bad - avoid unwrap() in library code
pub async fn fetch_instances(&self) -> Vec<Instance> {
    let response = self.client.get(url).await.unwrap();
    response.json().await.unwrap()
}
```

### 5. Dependencies Management

#### Dependency Hygiene
```bash
# Check for outdated dependencies
cargo outdated

# Check for security vulnerabilities
cargo audit

# Check for unused dependencies
cargo machete

# Update dependencies carefully
cargo update
```

**Requirements:**
- Keep dependencies up to date (security patches especially)
- Minimize dependency count - every dependency adds maintenance burden
- Use feature flags to make heavy dependencies optional when possible
- Document why each dependency is needed in internal docs
- Pin major versions in `Cargo.toml`
- Test after dependency updates

#### Current Dependencies Review
Based on `Cargo.toml`:
- `hyper`, `tokio`, `futures` - Core async HTTP (good choices)
- `serde`, `serde_json` - Serialization (essential)
- `url` - URL parsing (lightweight, good)
- `log` - Logging facade (good choice)
- `option-filter` - Helper utilities (verify if needed)

### 6. Async/Await Best Practices

Since this is an async library using Tokio:

**Requirements:**
- Use `async`/`.await` consistently
- Avoid blocking operations in async contexts
- Use `tokio::spawn` for concurrent operations
- Handle cancellation gracefully
- Document Send/Sync requirements
- Use appropriate Tokio features (already using `features = ["full"]`)

**Example:**
```rust
/// Fetches instances asynchronously with timeout.
///
/// # Arguments
///
/// * `timeout` - Maximum duration to wait for response
pub async fn fetch_with_timeout(
    &self,
    timeout: Duration
) -> Result<Vec<Instance>> {
    tokio::time::timeout(timeout, self.fetch_instances())
        .await
        .map_err(|_| Error::Timeout)?
}
```

### 7. API Stability

As a library crate, API stability is critical:

**Requirements:**
- Follow semantic versioning strictly (already using 0.1.1)
- Use `#[deprecated]` attribute when removing old APIs
- Document breaking changes in CHANGELOG
- Consider feature flags for experimental APIs
- Keep public API surface minimal
- Use builder patterns for complex configurations

### 8. Performance Considerations

**Requirements:**
- Prefer `&str` over `String` where possible
- Use `Cow<str>` when cloning might be avoided
- Avoid unnecessary allocations in hot paths
- Use `cargo bench` for performance-critical code
- Profile before optimizing
- Document performance characteristics of public APIs

### 9. CI/CD Requirements

Every CI pipeline MUST run:

```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Clippy with warnings as errors
cargo clippy --all-targets --all-features -- -D warnings

# 3. Build
cargo build --all-features

# 4. Test
cargo test --all-features

# 5. Documentation
cargo doc --no-deps --all-features

# 6. Security audit (optional but recommended)
cargo audit
```

### 10. Pre-Commit Checklist

Before committing any code, ensure:

- [ ] Code is formatted: `cargo fmt --all`
- [ ] Clippy passes: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] All tests pass: `cargo test --all-features`
- [ ] New public APIs are documented
- [ ] New functionality has tests (unit + integration)
- [ ] Error paths are tested
- [ ] No `unwrap()` or `expect()` in library code
- [ ] CHANGELOG updated (if applicable)
- [ ] Examples updated (if API changed)

### 11. Code Review Guidelines

When reviewing PRs:

- [ ] Verify all pre-commit checks pass
- [ ] Check test coverage for new code
- [ ] Verify documentation completeness
- [ ] Review error handling patterns
- [ ] Check for proper async/await usage
- [ ] Verify no breaking changes (or properly versioned)
- [ ] Ensure performance implications are understood
- [ ] Validate dependency additions are justified

## Quick Command Reference

```bash
# Development workflow
cargo fmt --all                                           # Format code
cargo clippy --all-targets --all-features -- -D warnings  # Lint code
cargo test --all-features                                 # Run tests
cargo doc --no-deps --open                                # Generate docs

# Advanced checks
cargo test --all-features -- --nocapture                  # Tests with output
cargo build --release                                     # Release build
cargo tree                                                # Dependency tree
cargo outdated                                            # Check updates

# Before release
cargo publish --dry-run                                   # Verify package
cargo package --list                                      # Check package contents
```

## Project-Specific Notes

### Eureka Client Specifics

This library implements a Netflix Eureka client, so:

1. **Network resilience** is critical - handle timeouts, retries, circuit breaking
2. **Service discovery** requires background refresh - consider tokio tasks
3. **Registration heartbeats** must be reliable - document guarantees
4. **Configuration** should be flexible - use builder pattern
5. **Examples** should show common patterns (register, discover, heartbeat)

### Test Environment

Consider providing:
- Mock Eureka server for integration tests
- Example docker-compose setup for local testing
- Documentation for testing against real Eureka instances

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Rustfmt Configuration](https://rust-lang.github.io/rustfmt/)
- [Async Rust Book](https://rust-lang.github.io/async-book/)
- [Tokio Documentation](https://tokio.rs/)

## Questions?

When in doubt:
1. Prioritize correctness over cleverness
2. Prefer explicit over implicit
3. Document everything public
4. Test everything thoroughly
5. Handle errors gracefully

---

*This document should be updated as the project evolves. All contributors and AI agents should follow these guidelines to maintain code quality and consistency.*
