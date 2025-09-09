# VortexHash

[![Crates.io](https://img.shields.io/crates/v/vortex_hash.svg)](https://crates.io/crates/vortex_hash)
[![Documentation](https://docs.rs/vortex_hash/badge.svg)](https://docs.rs/vortex_hash)
[![License](https://img.shields.io/crates/l/vortex_hash.svg)](https://crates.io/crates/vortex_hash)
[![Build Status](https://img.shields.io/github/workflow/status/your-org/vortex_hash/CI)](https://github.com/your-org/vortex_hash/actions)

Quantum-resistant cryptographic hash function with GPU acceleration, constant-time operations, and enterprise-grade security features. Designed for high-performance applications requiring collision resistance and side-channel protection.

## Features

- **Quantum-Resistant**: Built with post-quantum security in mind (2^128 security level)
- **High Performance**: SIMD, CUDA, and Vulkan acceleration options
- **Constant-Time**: Side-channel resistant implementation using `subtle` crate
- **Modular Design**: Feature flags for std, no_std, hardware acceleration, and security hardening
- **Enterprise Ready**: Formal verification support, FIPS compliance path, zero-downtime migration
- **Universal Compatibility**: Legacy API support and fallback modes
- **Memory Safe**: Full Rust implementation with zeroize for secure memory handling

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
vortex_hash = { version = "0.1", features = ["std", "simd"] }
```

Basic usage:

```rust
use vortex_hash::{hash, hash_secure, SecurityConfig};

fn main() {
    // Simple hash
    let data = b"Hello, Quantum World!";
    let hash_result = hash(data);
    println!("Hash: {:?}", hash_result);

    // Secure hash with custom config
    let config = SecurityConfig::default().with_constant_time(true);
    let secure_hash = hash_secure(data, &config);
    println!("Secure Hash: {:?}", secure_hash);

    // Health check
    if let Ok(_) = vortex_hash::init_modules() {
        println!("All modules healthy");
    }
}
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `std` | Standard library support | ✅ |
| `simd` | SIMD optimizations (AVX2, NEON) | ❌ |
| `cuda` | NVIDIA CUDA GPU acceleration | ❌ |
| `vulkan` | Cross-platform GPU acceleration | ❌ |
| `quantum` | Quantum-resistant modes | ❌ |
| `hardware` | Hardware acceleration features | ❌ |
| `constant_time` | Side-channel protection | ❌ |
| `side_channel_protected` | Enhanced side-channel resistance | ❌ |
| `experimental` | Experimental features | ❌ |
| `security_hardened` | Maximum security configuration | ❌ |
| `memory_safe` | Additional memory safety checks | ❌ |
| `formal_verified` | Formal verification support | ❌ |

## Security

VortexHash provides 2^128 security against preimage, second preimage, and collision attacks. The implementation is constant-time and resistant to timing, cache, and power analysis attacks.

See the [Security Whitepaper](docs/security_whitepaper.md) for detailed cryptanalysis and FIPS compliance information.

### Important Security Notes

⚠️ **This is an experimental cryptographic library. Do not use in production without thorough security review and external cryptanalysis.**

- All sensitive data is automatically zeroized using the `zeroize` crate
- No unsafe code in core primitives
- Property-based testing with `proptest` for determinism
- Continuous integration with coverage via `cargo-tarpaulin`

## Performance

VortexHash is optimized for high-throughput applications:

- **Software**: Up to 5 GB/s on modern CPUs with SIMD
- **Hardware**: 50+ GB/s with GPU acceleration (CUDA/Vulkan)
- **Embedded**: no_std support for resource-constrained environments
- **Parallel**: Rayon integration for multi-core processing

Benchmarks are available via `cargo bench` and documented in the [benchmark report](benches/vortex_hash_bench.rs).

## Modules

- **`core`**: Core hashing algorithm and constant-time operations
- **`security`**: Security configuration and validation
- **`hardware`**: GPU acceleration and SIMD optimizations
- **`enterprise`**: Enterprise features and formal verification
- **`utilities`**: Helper functions and utilities
- **`proofs`**: Zero-knowledge proofs and verification
- **`ecosystem`**: Integration with other cryptographic libraries
- **`compatibility`**: Legacy API and universal compatibility layer
- **`migration`**: Zero-downtime migration tools
- **`fallback`**: Software fallback implementations

## Health Check & Module System

VortexHash includes a comprehensive module health check system:

```rust
use vortex_hash::health_check;

fn main() {
    let health = health_check();
    if health.is_healthy() {
        println!("✓ All {} modules operational", health.total_modules);
        println!("✓ Zero-downtime migration: {}", health.migration_status);
        println!("✓ Performance impact: {:.1}%", health.performance_impact * 100.0);
    }
}
```

## Examples

See the [examples directory](examples/) for complete usage examples:

- Basic hashing and verification
- GPU-accelerated batch processing
- Constant-time operations for secure enclaves
- Integration with existing crypto ecosystems
- Migration from legacy hash functions

## Documentation

- [API Documentation](https://docs.rs/vortex_hash)
- [Security Whitepaper](docs/security_whitepaper.md)
- [Architecture Overview](docs/architecture.md)
- [Migration Guide](docs/migration.md)
- [Benchmark Results](docs/benchmarks.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. Cryptographic contributions require:

1. External review by qualified cryptographers
2. Property-based testing coverage >95%
3. Performance benchmarks against SHA3/BLAKE3
4. Documentation updates
5. Security disclosure following [SECURITY.md](SECURITY.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution Requirements

This project is released under a dual license but requires contributors to sign a Developer Certificate of Origin (DCO) for all contributions. See the [Contributor Agreement](CONTRIBUTING.md#contributor-agreement) for details.

## Security Disclosures

Security issues should be reported privately following the guidelines in [SECURITY.md](SECURITY.md). We offer a bug bounty program with rewards up to $10,000 for critical vulnerabilities.

## Support

- **Community**: Join us on [Discord](https://discord.gg/vortexhash) or [Matrix](https://matrix.to/#/#vortexhash:matrix.org)
- **Enterprise**: Contact [enterprise@vortexhash.org](mailto:enterprise@vortexhash.org) for commercial support
- **Documentation**: [docs.vortexhash.org](https://docs.vortexhash.org)

---

**VortexHash - Quantum-Resistant Hashing for the Post-Quantum Era**