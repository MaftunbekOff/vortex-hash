# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial VortexHash implementation with quantum-resistant security
- Hardware acceleration support (SIMD, CUDA, Vulkan)
- Comprehensive benchmarking framework vs BLAKE3
- Energy efficiency metrics and performance analysis
- CI/CD automation for continuous benchmarking
- Audit-ready report generation (JSON/CSV/Markdown)

### Changed
- Improved constant-time operations for side-channel resistance
- Enhanced streaming implementation for large datasets
- Optimized HMAC and secure hash modes

### Fixed
- Compilation issues with quantum_link integration
- Import resolution in test modules
- Benchmark configuration for accurate measurements

### Security
- Added side-channel protection features
- Implemented zeroization for sensitive data
- Baseline performance validation (>0.45 GB/s)

## [0.1.0] - 2025-01-15

### Added
- Core VortexHash algorithm implementation
- Basic security configuration options
- Initial test suite with property-based testing
- Performance benchmarks using Criterion
- Documentation and API reference

### Security
- Quantum-resistant construction based on lattice problems
- Constant-time arithmetic to prevent timing attacks
- Secure memory handling with zeroize crate

[0.1.0]: https://github.com/your-org/vortex_hash/compare/v0.0.0...v0.1.0