# Contributing to VortexHash

Thank you for your interest in contributing to VortexHash! We welcome contributions that improve the quality, performance, and security of this cryptographic library.

## Cryptographic Contribution Guidelines

Due to the sensitive nature of cryptographic software, all contributions must follow strict guidelines:

### 1. Security Review Required
- All cryptographic changes require external review by qualified cryptographers
- Implementations must include comprehensive test coverage (>95%)
- Performance benchmarks must be provided against established standards (SHA3, BLAKE3)

### 2. Testing Requirements
- Unit tests for all public functions
- Property-based testing with `proptest`
- Fuzz testing for input validation
- Performance benchmarks with `criterion`
- Coverage analysis with `cargo-tarpaulin`

### 3. Documentation Standards
- All public functions must have comprehensive documentation
- Security considerations must be clearly documented
- Usage examples for complex features
- Update README.md for new features

### 4. Code Style
- Follow Rust API guidelines
- Use `cargo fmt` and `cargo clippy`
- No unsafe code in core cryptographic primitives
- Comprehensive error handling
- Zeroize sensitive data automatically

## Development Setup

### Prerequisites
- Rust 1.75+ (stable)
- `cargo install cargo-fmt cargo-clippy`
- `cargo install criterion proptest cargo-tarpaulin`

### Building and Testing
```bash
# Clone the repository
git clone https://github.com/your-org/vortex_hash.git
cd vortex_hash

# Format code
cargo fmt

# Lint code
cargo clippy --all-features -- -D warnings

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench

# Check coverage
cargo tarpaulin --all-features --out Html
```

### Feature Testing
```bash
# Test with different feature combinations
cargo test --features "std simd constant_time"
cargo test --no-default-features
cargo test --features "no_std quantum"
```

## Contribution Process

### 1. Fork and Clone
1. Fork the repository on GitHub
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/vortex_hash.git`
3. Create a feature branch: `git checkout -b feature/your-feature`

### 2. Development Workflow
```bash
# Ensure clean state
git pull upstream main
cargo fmt
cargo clippy --fix --allow-dirty

# Make your changes
# Add tests for new functionality
# Update documentation

# Commit with conventional format
git commit -m "feat: add new constant-time multiplication

This implements constant-time multiplication for the core
hashing primitive using the subtle crate.

Fixes #123"
```

### 3. Pull Request Requirements
- Clear title following conventional commits
- Detailed description of changes
- Test results and benchmarks
- Documentation updates
- Link to related issues
- Security impact assessment

### 4. Security Disclosures
Security vulnerabilities should be reported privately following our [security policy](SECURITY.md). Do not open public issues for security concerns.

## Developer Certificate of Origin (DCO)

All contributions must include a signed DCO. By making a contribution, you certify that:

1. You have the right to submit the contribution under the license terms
2. You are willing to license the contribution under the Apache-2.0 or MIT license
3. You agree to the Contributor License Agreement

## Cryptographic Review Process

### Major Cryptographic Changes
1. **Design Review**: Submit RFC with algorithm design and security analysis
2. **Implementation**: Reference implementation with comprehensive tests
3. **External Review**: At least two independent cryptographic reviews
4. **Benchmarking**: Performance comparison with established algorithms
5. **Integration**: Merge after all reviews and automated tests pass

### Minor Changes
1. **Code Review**: Standard pull request review process
2. **Testing**: Full test suite must pass
3. **Documentation**: All changes documented
4. **Security Assessment**: Impact on security properties assessed

## Code Owners

Cryptographic core: `@crypto-review-team`
Performance optimizations: `@perf-team`
Documentation: `@docs-team`
Testing infrastructure: `@test-team`

## Release Process

Releases follow semantic versioning:

- **PATCH** releases: Bug fixes, documentation improvements
- **MINOR** releases: New features with backward compatibility
- **MAJOR** releases: Breaking changes or security fixes

Before each release:
1. Update CHANGELOG.md
2. Run full test suite across all supported platforms
3. Update documentation
4. Tag the release
5. Publish to crates.io

## Support

For questions about contributing:
- Open an issue with the `question` label
- Join our development Discord: [discord.gg/vortexhash](https://discord.gg/vortexhash)
- Email: dev@vortexhash.org

We appreciate your contributions to making VortexHash more secure and performant!