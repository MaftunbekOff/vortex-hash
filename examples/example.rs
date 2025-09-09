use vortex_hash::{SecurityConfig, VortexHash};

fn main() {
    // Basic hash
    let data = b"Hello, VortexHASH!";
    let basic_hash = VortexHash::hash(data);
    println!("Basic hash: {:?}", basic_hash);

    // Hash with custom config
    let config = SecurityConfig {
        rounds: 128,
        security_level: 512,
        constant_time: true,
        side_channel_protection: true,
    };
    let secure_hash = VortexHash::hash_secure(data, &config);
    println!("Secure hash: {:?}", secure_hash);

    // HMAC example
    let key = b"secret_key";
    let message = b"message to sign";
    let hmac_result = VortexHash::hmac(key, message);
    println!("HMAC: {:?}", hmac_result);

    // Constant time equality
    use vortex_hash::constant_time::ct_eq;
    let eq = ct_eq(&basic_hash, &secure_hash);
    println!("Constant time equality: {}", bool::from(eq));
}

#[cfg(test)]
mod tests {
    use super::*;
    use vortex_hash::VortexHash;

    #[test]
    fn test_basic_usage() {
        let data = b"test";
        let hash = VortexHash::hash(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hmac_usage() {
        let key = b"key";
        let data = b"data";
        let hmac_result = VortexHash::hmac(key, data);
        assert_eq!(hmac_result.len(), 32);
    }
}