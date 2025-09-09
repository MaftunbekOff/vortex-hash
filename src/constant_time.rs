use subtle::{Choice, ConstantTimeEq};

/// Constant-time equality check
pub fn ct_eq(a: &[u8], b: &[u8]) -> Choice {
    if a.len() != b.len() {
        Choice::from(0u8)
    } else {
        a.ct_eq(b)
    }
}