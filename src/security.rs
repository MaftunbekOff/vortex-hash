use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    pub rounds: u32,
    pub security_level: u32,
    pub constant_time: bool,
    pub side_channel_protection: bool,
}


impl SecurityConfig {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            rounds: 64,
            security_level: 256,
            constant_time: true,
            side_channel_protection: true,
        }
    }

    #[inline(always)]
    pub fn validate(&self) -> bool {
        self.rounds > 0 && self.security_level >= 128 && self.constant_time
    }
}

impl fmt::Display for SecurityConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SecurityConfig(rounds={}, level={}, ct={})",
            self.rounds, self.security_level, self.constant_time
        )
    }
}