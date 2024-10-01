use std::ops::{Add, Div, Mul, Rem, Sub};
use std::{fmt, u64};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uint256(pub [u64; 4]);

impl Uint256 {
    pub const ZERO: Self = Self([0, 0, 0, 0]);
    pub const ONE: Self = Self([1, 0, 0, 0]);
    pub const TWO: Self = Self([2, 0, 0, 0]);
    pub const MAX: Self = Self([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);

    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }

    pub fn set_zero(&mut self) {
        self.0 = [0, 0, 0, 0];
    }

    pub fn from_hex<S: AsRef<str>>(hex_str: S) -> Result<Self, String> {
        let hex_str = hex_str.as_ref();
        let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        // let hex_str = hex_str.trim_start_matches('0');

        if hex_str.len() != 64 {
            return Err("hex string too long".into());
        }

        let mut result = Self::ZERO;
        for (i, chunk) in hex_str.as_bytes().rchunks(16).enumerate() {
            let chunk_str = match std::str::from_utf8(chunk) {
                Ok(s) => s,
                Err(_) => return Err("Invalid utf8".into()),
            };
            let value = match u64::from_str_radix(chunk_str, 16) {
                Ok(v) => v,
                Err(_) => return Err("Invalid hex".into()),
            };
            result.0[i] = value;
        }

        Ok(result)
    }
}

impl fmt::Debug for Uint256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Uint256(0x{:016x}{:016x}{:016x}{:016x})",
            self.0[3], self.0[2], self.0[1], self.0[0]
        )
    }
}

impl Add for Uint256 {
    type Output = Self;

    // self.0 is just the internal array since it's not named
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self([0, 0, 0, 0]);
        let mut carry = 0u64;

        for i in 0..4 {
            let (sum, overflow1) = self.0[i].overflowing_add(rhs.0[i]);
            let (sum, overflow2) = sum.overflowing_add(carry);
            result.0[i] = sum;
            carry = if overflow1 || overflow2 { 1 } else { 0 };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Uint256([1, 0, 0, 0]);
        let b = Uint256([2, 0, 0, 0]);
        assert_eq!(a + b, Uint256([3, 0, 0, 0]));

        let max_u64 = Uint256([u64::MAX, 0, 0, 0]);
        let one = Uint256([1, 0, 0, 0]);
        assert_eq!(max_u64 + one, Uint256([0, 1, 0, 0]));

        let almost_max = Uint256([u64::MAX - 1, u64::MAX, u64::MAX, u64::MAX]);
        let one = Uint256::ONE;
        assert_eq!(almost_max + one, Uint256::MAX);

        assert_eq!(Uint256::MAX + Uint256::ONE, Uint256::ZERO);
        assert_eq!(Uint256::MAX + Uint256::ZERO, Uint256::MAX);
    }
}
