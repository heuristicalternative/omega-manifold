#![allow(dead_code)]

// Groth16 batched pairing verifier for state transition proofs.

#[allow(dead_code)]
pub const FIELD_MODULUS: u64 = 2305843009213693951; // 2^61 - 1

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Groth16Proof {
    pub a: (u64, u64),
    pub b: (u64, u64),
    pub c: (u64, u64),
    pub public_inputs: Vec<u64>,
}

impl Groth16Proof {
    pub fn is_canonical(&self) -> bool {
        self.a.0 < FIELD_MODULUS
            && self.a.1 < FIELD_MODULUS
            && self.b.0 < FIELD_MODULUS
            && self.b.1 < FIELD_MODULUS
            && self.c.0 < FIELD_MODULUS
            && self.c.1 < FIELD_MODULUS
            && self.public_inputs.iter().all(|x| *x < FIELD_MODULUS)
    }

    /// A valid proof satisfies the mock pairing equation:
    /// A * B == C + sum(public_inputs) (mod FIELD_MODULUS).
    pub fn pairing_holds(&self, modulus: u64) -> bool {
        let lhs = (self.a.0 as u128 * self.b.0 as u128) % modulus as u128;
        let rhs_inputs: u64 = self.public_inputs.iter().fold(0u64, |acc, x| {
            acc.wrapping_add(*x) % modulus
        });
        let rhs = (self.c.0 as u128 + rhs_inputs as u128) % modulus as u128;
        lhs == rhs
    }
}

#[allow(dead_code)]
pub struct Groth16Verifier {
    pub modulus: u64,
}

impl Groth16Verifier {
    pub fn new() -> Self {
        Self {
            modulus: FIELD_MODULUS,
        }
    }

    pub fn verify_proof(&self, proof: &Groth16Proof) -> Result<bool, String> {
        if !proof.is_canonical() {
            return Err("non-canonical field element".to_string());
        }
        Ok(proof.pairing_holds(self.modulus))
    }

    pub fn verify_batched_proofs(&self, proofs: &[Groth16Proof]) -> Result<bool, String> {
        if proofs.is_empty() {
            return Ok(true);
        }
        for proof in proofs {
            if !proof.is_canonical() {
                return Err("non-canonical field element in batch".to_string());
            }
            if !proof.pairing_holds(self.modulus) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_proof() -> Groth16Proof {
        Groth16Proof {
            a: (2, 3),
            b: (4, 5),
            c: (8, 7), // 2*4=8, inputs sum 0 => c.0=8
            public_inputs: vec![],
        }
    }

    #[test]
    fn test_valid_proof_accepted() {
        let verifier = Groth16Verifier::new();
        assert!(verifier.verify_proof(&valid_proof()).unwrap());
    }

    #[test]
    fn test_invalid_curve_point_rejected() {
        let verifier = Groth16Verifier::new();
        let mut proof = valid_proof();
        proof.a.0 = FIELD_MODULUS; // non-canonical
        let result = verifier.verify_proof(&proof);
        assert!(result.is_err());
    }

    #[test]
    fn test_batched_verification() {
        let verifier = Groth16Verifier::new();
        let proofs = vec![valid_proof(), valid_proof()];
        assert!(verifier.verify_batched_proofs(&proofs).unwrap());
    }

    #[test]
    fn test_batched_invalid_returns_false() {
        let verifier = Groth16Verifier::new();
        let mut bad = valid_proof();
        bad.c.0 = 999; // pairing fails
        let proofs = vec![valid_proof(), bad];
        assert!(!verifier.verify_batched_proofs(&proofs).unwrap());
    }

    #[test]
    fn test_batched_throughput() {
        let verifier = Groth16Verifier::new();
        let proofs: Vec<Groth16Proof> = (0..1000).map(|_| valid_proof()).collect();
        assert!(verifier.verify_batched_proofs(&proofs).unwrap());
    }
}
