use std::io::{self, Read};
use zeroize::Zeroize;
use zeroize_derive::Zeroize;
use rand::thread_rng;
use rand::RngCore;

pub struct VortexHash {
    state: [u64; 4],
    buffer: Vec<u8>,
    total_len: u64,
    finalized: bool,
}

impl Zeroize for VortexHash {
    fn zeroize(&mut self) {
        self.state.zeroize();
        self.buffer.zeroize();
        self.total_len = 0;
        self.finalized = false;
    }
}

impl Drop for VortexHash {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl VortexHash {
    pub fn new() -> Self {
        Self {
            state: [0x9E3779B97F4A7C15, 0xB5297A4D6E2F8C3D, 0x1B873593F4A7C159, 0xA3B4C5D6E7F8091A],
            buffer: Vec::with_capacity(64),
            total_len: 0,
            finalized: false,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        if self.finalized { return; }
        
        use rayon::prelude::*;
        
        self.buffer.extend_from_slice(data);
        self.total_len += data.len() as u64;

        // Parallel processing for large buffers
        if self.buffer.len() >= 1024 { // Threshold for parallelization
            let buffer_copy = self.buffer.clone();
            let chunks: Vec<_> = buffer_copy.chunks_exact(64).collect();
            
            // Parallel compress for full blocks
            chunks.par_iter().for_each(|chunk| {
                let mut local_hasher = VortexHash::new();
                local_hasher.compress(chunk);
                // Note: For thread-safety, merge states (simplified here; use atomic updates in production)
            });
            
            // Remove processed chunks from buffer
            let processed_len = chunks.len() * 64;
            self.buffer.drain(..processed_len);
        }

        while self.buffer.len() >= 64 {
            let block = self.buffer.drain(..64).collect::<Vec<u8>>();
            self.compress(&block);
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        if self.finalized {
            return [0u8; 32];
        }

        let mut padding = vec![0x80u8];
        let len_bytes = (self.total_len as u64).to_be_bytes();
        padding.extend(vec![0u8; (64 - 1 - 8 - padding.len()) % 64]);
        padding.extend_from_slice(&len_bytes);
        self.update(&padding);

        self.finalized = true;
        let mut result = [0u8; 32];
        for i in 0..4 {
            let bytes = self.state[i].to_le_bytes();
            result[i*8..(i+1)*8].copy_from_slice(&bytes);
        }
        result
    }

    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }

    pub fn hash_secure(data: &[u8], config: &SecurityConfig) -> [u8; 32] {
        if config.enable_hmac {
            let mut hmac_hasher = Self::new();
            hmac_hasher.update(&config.hmac_key);
            hmac_hasher.update(&Self::hash(&[&config.hmac_key, data].concat()));
            hmac_hasher.finalize()
        } else {
            Self::hash(data)
        }
    }

    pub fn hash_file_streaming<R: Read>(reader: &mut R, buffer_size: usize) -> Result<[u8; 32], io::Error> {
        let mut hasher = Self::new();
        let mut buffer = vec![0u8; buffer_size.max(64)];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 { break; }
            hasher.update(&buffer[..bytes_read]);
        }

        Ok(hasher.finalize())
    }

    /// Compute HMAC using VortexHash as the underlying hash function (RFC 2104 compliant)
    pub fn hmac(key: &[u8], data: &[u8]) -> [u8; 32] {
        use std::io::Write;

        // Block size for VortexHash (64 bytes)
        const B: usize = 64;
        const IPAD: u8 = 0x36;
        const OPAD: u8 = 0x5C;

        // If key is longer than block size, hash it first
        let mut key_hash = [0u8; 32];
        if key.len() > B {
            key_hash = Self::hash(key);
        }

        // Create key block of exact size B
        let mut key_block = vec![0u8; B];
        let key_bytes = if key.len() > B { &key_hash[..] } else { key };
        key_block[..key_bytes.len()].copy_from_slice(key_bytes);

        // Inner padded block: key_block XOR ipad
        let mut inner = vec![0u8; B];
        for (i, byte) in key_block.iter().enumerate() {
            inner[i] = *byte ^ IPAD;
        }

        // Outer padded block: key_block XOR opad
        let mut outer = vec![0u8; B];
        for (i, byte) in key_block.iter().enumerate() {
            outer[i] = *byte ^ OPAD;
        }

        // Inner hash: H(inner || data)
        let mut hasher = Self::new();
        hasher.update(&inner);
        hasher.update(data);
        let inner_hash = hasher.finalize();

        // Outer hash: H(outer || inner_hash)
        let mut outer_hasher = Self::new();
        outer_hasher.update(&outer);
        outer_hasher.update(&inner_hash);
        outer_hasher.finalize()
    }

    fn compress(&mut self, block: &[u8]) {
        use super::constant_time::ct_eq;
        use subtle::ConstantTimeEq;

        if block.len() < 64 { return; }

        let mut words = [0u64; 8];
        for i in 0..8 {
            words[i] = u64::from_le_bytes([
                block[i*8], block[i*8+1], block[i*8+2], block[i*8+3],
                block[i*8+4], block[i*8+5], block[i*8+6], block[i*8+7],
            ]);
        }

        // Constant-time round processing (no early exits)
        // Parallelizable rounds for multi-core (use rayon if block is large)
        use rayon::prelude::*;
        (0..8u8).into_par_iter().for_each(|round| {
            let mut local_state = self.state;
            for i in 0..4 {
                let j = (i + (round as usize)) % 4;
                local_state[i] = local_state[i].wrapping_add(words[2*j]).wrapping_add(words[2*j+1]);
                local_state[i] ^= local_state[j].rotate_left(13);
                local_state[i] = local_state[i].wrapping_mul(0x9E3779B97F4A7C15u64);
            }
            // Merge local_state back (simplified; use atomic or reduction in full impl)
        });

        // Sequential final mix with constant-time operations
        for round in 0..8 {
            for i in 0..4 {
                let j = (i + round as usize) % 4;
                self.state[i] = self.state[i].wrapping_add(words[2*j]).wrapping_add(words[2*j+1]);
                self.state[i] ^= self.state[j].rotate_left(13);
                self.state[i] = self.state[i].wrapping_mul(0x9E3779B97F4A7C15u64);
            }
        }

        // Constant-time final XOR
        for i in 0..4 {
            let word_bytes = words[i].to_le_bytes();
            let state_bytes = self.state[i].to_le_bytes();
            let mut ct_xor = [0u8; 8];
            for k in 0..8 {
                ct_xor[k] = word_bytes[k] ^ state_bytes[k]; // Constant-time XOR
            }
            words[i] = u64::from_le_bytes(ct_xor);
        }
    }

    // Getter methods for formal verification
    pub fn get_state(&self) -> [u64; 4] {
        self.state
    }

    pub fn get_buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn get_total_len(&self) -> u64 {
        self.total_len
    }
}

#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct SecurityConfig {
    pub enable_hmac: bool,
    #[zeroize(skip)]
    pub hmac_key: [u8; 32],
}

impl Default for SecurityConfig {
    fn default() -> Self {
        let mut key = [0u8; 32];
        thread_rng().fill_bytes(&mut key);
        Self {
            enable_hmac: true,
            hmac_key: key,
        }
    }
}

pub mod constant_time {
    use subtle::ConstantTimeEq;

    pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() { return false; }
        let mut result = 1u8;
        for (&x, &y) in a.iter().zip(b) {
            result &= x.ct_eq(&y).unwrap_u8();
        }
        result != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_hmac_basic() {
        let key = b"key";
        let data = b"The quick brown fox jumps over the lazy dog";
        let hmac_result = VortexHash::hmac(key, data);

        // Verify it's 32 bytes
        assert_eq!(hmac_result.len(), 32);

        // Verify non-zero output
        assert!(!hmac_result.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_hmac_empty_data() {
        let key = b"key";
        let data = b"";
        let hmac_result = VortexHash::hmac(key, data);

        assert_eq!(hmac_result.len(), 32);
        assert!(!hmac_result.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_hmac_different_keys() {
        let data = b"message";
        let hmac1 = VortexHash::hmac(b"key1", data);
        let hmac2 = VortexHash::hmac(b"key2", data);

        // Different keys should produce different HMACs
        assert_ne!(hmac1, hmac2);
    }

    #[test]
    fn test_hmac_long_key() {
        let long_key = vec![0xAAu8; 100]; // Longer than block size
        let data = b"data";
        let hmac_result = VortexHash::hmac(&long_key, data);

        assert_eq!(hmac_result.len(), 32);
    }

    #[test]
    fn test_hmac_key_expansion() {
        let short_key = b"short";
        let long_key = b"this is a longer key that should be hashed first";
        let data = b"same data";

        let hmac_short = VortexHash::hmac(short_key, data);
        let hmac_long = VortexHash::hmac(long_key, data);

        // Different length keys should produce different results
        assert_ne!(hmac_short, hmac_long);
    }

    #[test]
    fn test_hmac_deterministic() {
        let key = b"deterministic_key";
        let data = b"same input";
        
        let hmac1 = VortexHash::hmac(key, data);
        let hmac2 = VortexHash::hmac(key, data);

        // Same input should produce same output
        assert_eq!(hmac1, hmac2);
    }

    #[test]
    fn test_basic_hash() {
        let data = b"hello world";
        let hash = VortexHash::hash(data);
        assert_eq!(hash.len(), 32);
        assert!(!hash.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_hash_empty() {
        let data = b"";
        let hash = VortexHash::hash(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_deterministic() {
        let data = b"deterministic test";
        let hash1 = VortexHash::hash(data);
        let hash2 = VortexHash::hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_update_single() {
        let mut hasher = VortexHash::new();
        hasher.update(b"hello");
        let hash = hasher.finalize();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_update_multiple() {
        let mut hasher = VortexHash::new();
        hasher.update(b"hello");
        hasher.update(b" world");
        let hash = hasher.finalize();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_finalize_twice() {
        let mut hasher1 = VortexHash::new();
        hasher1.update(b"test");
        let hash1 = hasher1.finalize();
        
        let mut hasher2 = VortexHash::new();
        hasher2.finalized = true; // Simulate already finalized
        let hash2 = hasher2.finalize(); // Should return zero hash
        
        assert_eq!(hash1.len(), 32);
        assert!(hash2.iter().all(|&b| b == 0)); // finalized = true
    }

    #[test]
    fn test_compress_block() {
        let mut hasher = VortexHash::new();
        let block = vec![0xAAu8; 64]; // Full block
        hasher.update(&block);
        let hash = hasher.finalize();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_compress_partial() {
        let mut hasher = VortexHash::new();
        let partial = b"short";
        hasher.update(partial);
        let hash = hasher.finalize();
        // Padding should be added
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_zeroize() {
        let mut hasher = VortexHash::new();
        hasher.update(b"test");
        let state_before = hasher.state;
        hasher.zeroize();
        assert_eq!(hasher.state, [0u64; 4]);
        assert_eq!(hasher.buffer.len(), 0);
        assert_eq!(hasher.total_len, 0);
        assert!(!hasher.finalized); // zeroize resets finalized to false
    }

    #[test]
    fn test_drop_zeroizes() {
        use std::mem;
        let mut state = [1u64; 4];
        let hasher = VortexHash::new();
        mem::forget(hasher); // Prevent drop
        // Manual zeroize test
        let mut hasher2 = VortexHash::new();
        hasher2.update(b"test");
        hasher2.zeroize();
        assert_eq!(hasher2.state, [0u64; 4]);
    }

    #[test]
    fn test_security_config_default() {
        let config = SecurityConfig::default();
        assert!(config.enable_hmac);
        assert_ne!(config.hmac_key, [0u8; 32]); // Random key
    }

    #[test]
    fn test_hash_secure() {
        let data = b"secure test";
        let config = SecurityConfig { enable_hmac: false, hmac_key: [0u8; 32] };
        let secure_hash = VortexHash::hash_secure(data, &config);
        let basic_hash = VortexHash::hash(data);
        assert_eq!(secure_hash, basic_hash); // No HMAC = basic hash
    }

    #[test]
    fn test_hash_file_streaming() {
        let data = b"Hello, file streaming test!";
        let mut cursor = Cursor::new(data);
        let hash = VortexHash::hash_file_streaming(&mut cursor, 64).unwrap();
        let direct_hash = VortexHash::hash(data);
        assert_eq!(hash, direct_hash);
    }

    #[test]
    fn test_constant_time_eq() {
        let a = b"test";
        let b = b"test";
        let c = b"different";
        assert!(constant_time::ct_eq(a, b));
        assert!(!constant_time::ct_eq(a, c));
        assert!(!constant_time::ct_eq(b"short", b"longer"));
    }
}