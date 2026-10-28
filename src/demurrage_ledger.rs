#![allow(dead_code)]

// Gesellian continuous demurrage decay for token velocity enforcement.

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DemurrageAccount {
    pub id: String,
    pub balance: f64,
    pub last_active_epoch: u64,
    pub decay_rate: f64,
}

impl DemurrageAccount {
    pub fn new(id: &str, balance: f64, last_active_epoch: u64, decay_rate: f64) -> Self {
        Self {
            id: id.to_string(),
            balance,
            last_active_epoch,
            decay_rate,
        }
    }

    /// Continuously deduct demurrage based on elapsed epochs of stagnation.
    /// Returns the decayed value, which is routed into the common wealth pool.
    pub fn apply_demurrage(&mut self, current_epoch: u64) -> f64 {
        if current_epoch <= self.last_active_epoch {
            return 0.0;
        }
        let elapsed = (current_epoch - self.last_active_epoch) as f64;
        let decayed = (self.balance * self.decay_rate * elapsed)
            .min(self.balance)
            .max(0.0);
        self.balance -= decayed;
        self.last_active_epoch = current_epoch;
        decayed
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DemurrageLedger {
    pub accounts: Vec<DemurrageAccount>,
    pub common_wealth_pool: f64,
}

impl DemurrageLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_account(&mut self, account: DemurrageAccount) {
        self.accounts.push(account);
    }

    pub fn apply_demurrage(&mut self, current_epoch: u64) -> f64 {
        let mut total = 0.0;
        for account in &mut self.accounts {
            total += account.apply_demurrage(current_epoch);
        }
        self.common_wealth_pool += total;
        total
    }

    pub fn account(&self, id: &str) -> Option<&DemurrageAccount> {
        self.accounts.iter().find(|a| a.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demurrage_decay_over_time() {
        let mut account = DemurrageAccount::new("node_a", 100.0, 0, 0.1);
        let decayed = account.apply_demurrage(1);
        assert!((decayed - 10.0).abs() < 1e-9);
        assert!((account.balance - 90.0).abs() < 1e-9);
        assert_eq!(account.last_active_epoch, 1);

        let decayed2 = account.apply_demurrage(3);
        // 2 epochs * 0.1 * 90 = 18
        assert!((decayed2 - 18.0).abs() < 1e-9);
        assert!((account.balance - 72.0).abs() < 1e-9);
    }

    #[test]
    fn test_no_decay_when_not_elapsed() {
        let mut account = DemurrageAccount::new("node_b", 50.0, 5, 0.1);
        let decayed = account.apply_demurrage(5);
        assert!((decayed).abs() < 1e-9);
        assert!((account.balance - 50.0).abs() < 1e-9);
    }

    #[test]
    fn test_common_wealth_pool_accumulation() {
        let mut ledger = DemurrageLedger::new();
        ledger.register_account(DemurrageAccount::new("a", 100.0, 0, 0.1));
        ledger.register_account(DemurrageAccount::new("b", 200.0, 0, 0.05));

        let total = ledger.apply_demurrage(1);
        assert!((total - 20.0).abs() < 1e-9); // 10 + 10
        assert!((ledger.common_wealth_pool - 20.0).abs() < 1e-9);
    }

    #[test]
    fn test_decay_capped_at_balance() {
        let mut account = DemurrageAccount::new("node_c", 10.0, 0, 1.0);
        let decayed = account.apply_demurrage(5);
        assert!((decayed - 10.0).abs() < 1e-9);
        assert!((account.balance).abs() < 1e-9);
    }
}
