//! Utility functions for VortexHash

use sha2::{Sha256, Digest};

pub fn utils_hash(data: &[u8]) -> [u8; 32] {
   let mut hasher = Sha256::new();
   hasher.update(data);
   let result = hasher.finalize();
   let mut hash = [0u8; 32];
   hash.copy_from_slice(&result);
   hash
}

pub fn validate_input(data: &[u8]) -> bool {
   !data.is_empty()
}

pub mod constant_time {
   pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
       if a.len() != b.len() {
           return false;
       }
       let mut result = 0u8;
       for i in 0..a.len() {
           result |= a[i] ^ b[i];
       }
       result == 0
   }
}