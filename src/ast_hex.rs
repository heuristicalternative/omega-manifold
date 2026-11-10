use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexCoord6 {
    pub h: i32,
    pub k: i32,
    pub l: i32,
    pub m: i32,
    pub n: i32,
    pub p: i32,
}

impl HexCoord6 {
    pub const fn new(h: i32, k: i32, l: i32, m: i32, n: i32, p: i32) -> Self {
        Self { h, k, l, m, n, p }
    }

    pub fn validate_hyperplane(&self) -> bool {
        let sum = self.h as i64 + self.k as i64 + self.l as i64 + self.m as i64 + self.n as i64 + self.p as i64;
        sum == 0
    }

    pub fn rotate_60_degrees(&self) -> Self {
        Self {
            h: self.k,
            k: self.l,
            l: self.m,
            m: self.n,
            n: self.p,
            p: self.h,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_coord_invariants() {
        let coord = HexCoord6::new(1, -1, 2, -2, 0, 0);
        assert!(coord.validate_hyperplane(), "Hyperplane sum must be zero for H1=0");
        
        let rotated = coord.rotate_60_degrees();
        assert_eq!(rotated.h, -1);
        assert_eq!(rotated.p, 1);
    }
}
