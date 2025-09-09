use crate::security::SecurityConfig;

#[derive(Debug, Clone)]
pub struct VortexHash {
    state: [u8; 64], // Sponge state
    rate: usize,
}

impl VortexHash {
    pub fn new(_config: &SecurityConfig) -> Self {
        Self {
            state: [0u8; 64],
            rate: 32, // Bytes processed per block
        }
    }

    pub fn absorb(&mut self, data: &[u8]) {
        let mut offset = 0;
        while offset < data.len() {
            let chunk_size = std::cmp::min(self.rate, data.len() - offset);
            let chunk = &data[offset..offset + chunk_size];
            for (i, &byte) in chunk.iter().enumerate() {
                self.state[i] ^= byte;
            }
            offset += chunk_size;
            if offset % self.rate == 0 {
                self.permute();
            }
        }
    }

    pub fn squeeze(&mut self) -> [u8; 32] {
        self.permute();
        let mut output = [0u8; 32];
        output.copy_from_slice(&self.state[0..32]);
        output
    }

    fn permute(&mut self) {
        // Simple ARX-based permutation for demonstration
        for _ in 0..8 {
            // 8 rounds
            for i in 0..64 {
                self.state[i] = self.state[i].wrapping_add(self.state[(i + 1) % 64]);
                self.state[i] = self.state[i].rotate_left(13);
                self.state[i] ^= self.state[(i + 17) % 64];
            }
        }
    }

    pub fn hash_secure(data: &[u8], _config: &SecurityConfig) -> [u8; 32] {
        let mut hasher = VortexHash::new(&SecurityConfig::default());
        hasher.absorb(data);
        hasher.squeeze()
    }

    pub fn hmac(key: &[u8], data: &[u8]) -> [u8; 32] {
        let config = SecurityConfig::default();
        let mut hasher = VortexHash::new(&config);
        hasher.absorb(key);
        hasher.absorb(data);
        hasher.squeeze()
    }

    pub fn hash(data: &[u8]) -> [u8; 32] {
        let config = SecurityConfig::default();
        VortexHash::hash_secure(data, &config)
    }
}

pub fn hash_secure(data: &[u8], config: &SecurityConfig) -> [u8; 32] {
    VortexHash::hash_secure(data, config)
}