//! Migration helpers for VortexHash

use sha2::{Digest, Sha256};

pub struct MigrationHelper;

impl MigrationHelper {
    pub fn migrate_from_legacy(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

pub fn is_migration_needed() -> bool {
    false // Placeholder
}