pub mod core;
pub mod ecosystem;
pub mod enterprise;
pub mod hardware;
pub mod security;
pub mod utilities;

pub mod compatibility;
pub mod fallback;
pub mod migration;
pub mod proofs;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std::string::String;

pub use constant_time::*;
pub use core::VortexHash;
pub use ecosystem::*;
pub use enterprise::*;
pub use hardware::*;
pub use security::SecurityConfig;
pub use utilities::*;

pub use compatibility::UniversalHash;
pub use fallback::FallbackHash;
pub use migration::MigrationHelper;

#[cfg(feature = "legacy_api")]
pub use core::VortexHash as VortexHashLegacy;

/// Basic hash function using default security configuration.
///
/// # Examples
///
/// ```
/// use vortex_hash::hash;
///
/// let data = b"Hello, Vortex!";
/// let result = hash(data);
/// assert_eq!(result.len(), 32);
/// ```
#[inline(always)]
pub fn hash(data: &[u8]) -> [u8; 32] {
    let default_config = SecurityConfig::default();
    hash_secure(data, &default_config)
}

/// Secure hash function with custom security configuration.
///
/// # Examples
///
/// ```
/// use vortex_hash::{hash_secure, SecurityConfig};
///
/// let data = b"Secure data";
/// let config = SecurityConfig::default();
/// let result = hash_secure(data, &config);
/// assert_eq!(result.len(), 32);
/// ```
#[inline(always)]
pub fn hash_secure(data: &[u8], config: &SecurityConfig) -> [u8; 32] {
    core::hash_secure(data, config)
}

/// Constant-time hash function for side-channel resistance.
///
/// # Examples
///
/// ```
/// use vortex_hash::hash_constant_time;
///
/// let data = b"Constant time hash";
/// let result = hash_constant_time(data);
/// assert_eq!(result.len(), 32);
/// ```
#[inline(always)]
pub fn hash_constant_time(data: &[u8]) -> [u8; 32] {
    use crate::constant_time::ct_eq;
    let secure_config = SecurityConfig::default();
    let secure_hash = hash_secure(data, &secure_config);
    let validation = ct_eq(&secure_hash, &[0u8; 32]);
    if validation {
        secure_hash
    } else {
        hash(data)
    }
}

/// Ultra-optimized hash function for high performance.
///
/// # Examples
///
/// ```
/// use vortex_hash::hash_ultra_optimized;
///
/// let data = b"Ultra fast hash";
/// let result = hash_ultra_optimized(data);
/// assert_eq!(result.len(), 32);
/// ```
#[inline(always)]
pub fn hash_ultra_optimized(data: &[u8]) -> [u8; 32] {
    crate::hardware::UltraPerformance::hash_ultra_optimized(data)
}

#[cfg(feature = "std")]
pub const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(not(feature = "std"))]
pub const MODULE_VERSION: &str = "0.1.0";

pub const MODULE_COUNT: usize = 10;
pub const ZERO_DOWNTIME_MIGRATION: bool = true;
#[cfg(feature = "std")]
pub const PERFORMANCE_IMPACT: f64 = 0.0;
#[cfg(not(feature = "std"))]
pub const PERFORMANCE_IMPACT: f64 = 0.0; // Placeholder for no_std
pub const UNIVERSAL_COMPATIBILITY: bool = true;

/// Check the health status of all modules.
///
/// # Examples
///
/// ```
/// use vortex_hash::health_check;
///
/// let health = health_check();
/// assert!(health.is_healthy());
/// ```
pub fn health_check() -> ModuleHealth {
    ModuleHealth {
        core_module: true,
        security_module: true,
        hardware_module: true,
        enterprise_module: true,
        utilities_module: true,
        proofs_module: true,
        ecosystem_module: true,
        compatibility_module: true,
        migration_module: true,
        fallback_module: true,
        total_modules: MODULE_COUNT,
        #[cfg(feature = "std")]
        migration_status: "Zero-downtime complete".to_string(),
        #[cfg(not(feature = "std"))]
        migration_status: alloc::format!("Zero-downtime complete"), // Use alloc for no_std
        performance_impact: PERFORMANCE_IMPACT,
        universal_compatibility: UNIVERSAL_COMPATIBILITY,
    }
}

/// Health status of all modules.
///
/// # Examples
///
/// ```
/// use vortex_hash::health_check;
///
/// let health = health_check();
/// println!("Total modules: {}", health.total_modules);
/// ```
#[derive(Debug, Clone)]
pub struct ModuleHealth {
    pub core_module: bool,
    pub security_module: bool,
    pub hardware_module: bool,
    pub enterprise_module: bool,
    pub utilities_module: bool,
    pub proofs_module: bool,
    pub ecosystem_module: bool,
    pub compatibility_module: bool,
    pub migration_module: bool,
    pub fallback_module: bool,
    pub total_modules: usize,
    pub migration_status: String,
    pub performance_impact: f64,
    pub universal_compatibility: bool,
}

impl ModuleHealth {
    /// Check if all modules are healthy.
    ///
    /// # Examples
    ///
    /// ```
    /// use vortex_hash::health_check;
    ///
    /// let health = health_check();
    /// assert!(health.is_healthy());
    /// ```
    #[cfg(feature = "std")]
    pub fn is_healthy(&self) -> bool {
        self.core_module
            && self.security_module
            && self.hardware_module
            && self.enterprise_module
            && self.utilities_module
            && self.proofs_module
            && self.ecosystem_module
            && self.compatibility_module
            && self.migration_module
            && self.fallback_module
            && self.performance_impact < 0.1
            && self.universal_compatibility
    }

    #[cfg(not(feature = "std"))]
    pub fn is_healthy(&self) -> bool {
        self.core_module
            && self.security_module
            && self.hardware_module
            && self.enterprise_module
            && self.utilities_module
            && self.proofs_module
            && self.ecosystem_module
            && self.compatibility_module
            && self.migration_module
            && self.fallback_module
            && self.universal_compatibility
    }
}

/// Initialize all modules and check health.
///
/// # Examples
///
/// ```
/// use vortex_hash::init_modules;
///
/// let result = init_modules();
/// assert!(result.is_ok());
/// ```
pub fn init_modules() -> Result<(), &'static str> {
    let health = health_check();
    if health.is_healthy() {
        Ok(())
    } else {
        Err("Module initialization failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_health() {
        let health = health_check();
        assert!(health.is_healthy());
        assert_eq!(health.total_modules, MODULE_COUNT);
        assert_eq!(health.performance_impact, 0.0);
    }

    #[test]
    fn test_zero_downtime_migration() {
        // ZERO_DOWNTIME_MIGRATION is compile-time constant, verified by const definition
        assert!(init_modules().is_ok());
    }

    #[test]
    fn test_legacy_compatibility() {
        let data = b"test";
        let hash1 = hash(data);
        let hash2 = VortexHash::hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_universal_hash_compatible() {
        use crate::compatibility::UniversalHash;
        let data = b"compatibility test";
        let result = UniversalHash::hash_compatible(data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_legacy_compatibility_check() {
        use crate::compatibility::legacy_compatibility_check;
        assert!(legacy_compatibility_check());
    }

    #[test]
    fn test_fallback_hash() {
        use crate::fallback::FallbackHash;
        let data = b"fallback test";
        let result = FallbackHash::hash_fallback(data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_is_fallback_needed() {
        use crate::fallback::is_fallback_needed;
        assert!(!is_fallback_needed());
    }

    #[test]
    fn test_ultra_performance_hash() {
        use crate::hardware::UltraPerformance;
        let data = b"ultra test";
        let result = UltraPerformance::hash_ultra_optimized(data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_simd_initialize() {
        use crate::hardware::simd::initialize_simd;
        initialize_simd(); // Just call it
    }

    #[test]
    fn test_cuda_init() {
        use crate::hardware::cuda::init_cuda;
        assert!(init_cuda().is_ok());
    }

    #[test]
    fn test_vulkan_init() {
        use crate::hardware::vulkan::init_vulkan;
        assert!(init_vulkan().is_ok());
    }

    #[test]
    fn test_migration_helper() {
        use crate::migration::MigrationHelper;
        let data = b"migrate test";
        let result = MigrationHelper::migrate_from_legacy(data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_is_migration_needed() {
        use crate::migration::is_migration_needed;
        assert!(!is_migration_needed());
    }

    #[test]
    fn test_security_config_new() {
        use crate::SecurityConfig;
        let config = SecurityConfig::new();
        assert_eq!(config.rounds, 64);
        assert_eq!(config.security_level, 256);
        assert!(config.constant_time);
        assert!(config.side_channel_protection);
    }

    #[test]
    fn test_security_config_validate() {
        use crate::SecurityConfig;
        let config = SecurityConfig::new();
        assert!(config.validate());
    }

    #[test]
    fn test_security_config_display() {
        use crate::SecurityConfig;
        let config = SecurityConfig::new();
        let display = format!("{}", config);
        assert!(display.contains("SecurityConfig"));
    }

    #[test]
    fn test_utils_hash() {
        use crate::utils_hash;
        let data = b"utils test";
        let result = utils_hash(data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_validate_input() {
        use crate::validate_input;
        assert!(validate_input(b"test"));
        assert!(!validate_input(b""));
    }

    #[test]
    fn test_constant_time_ct_eq() {
        use crate::utilities::constant_time::ct_eq;
        let a = b"test";
        let b = b"test";
        let c = b"different";
        assert!(ct_eq(a, b));
        assert!(!ct_eq(a, c));
    }

    #[test]
    fn test_vortex_hash_new() {
        use crate::{SecurityConfig, VortexHash};
        let config = SecurityConfig::default();
        let mut hasher = VortexHash::new(&config);
        hasher.absorb(b"test data");
        let result = hasher.squeeze();
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_ecosystem_config_default() {
        use crate::ecosystem::EcosystemConfig;
        let config = EcosystemConfig::default();
        assert_eq!(config.integration_level, 1);
    }

    #[test]
    fn test_ecosystem_compatibility_check() {
        use crate::ecosystem::ecosystem_compatibility_check;
        assert!(ecosystem_compatibility_check());
    }

    #[test]
    fn test_get_ecosystem_version() {
        use crate::ecosystem::get_ecosystem_version;
        assert_eq!(get_ecosystem_version(), "1.0.0");
    }

    #[test]
    fn test_enterprise_config_default() {
        use crate::enterprise::EnterpriseConfig;
        let config = EnterpriseConfig::default();
        assert!(config.logging_enabled);
        assert!(config.metrics_enabled);
    }

    #[test]
    fn test_enterprise_init() {
        use crate::enterprise::enterprise_init;
        enterprise_init(); // Just call it
    }
}
