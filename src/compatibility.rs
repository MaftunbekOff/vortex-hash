//! Compatibility layer for VortexHash

use sha2::{Digest, Sha256};

pub struct UniversalHash;

impl UniversalHash {
    pub fn hash_compatible(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

pub fn legacy_compatibility_check() -> bool {
    true
}