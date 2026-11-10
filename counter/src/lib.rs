//! A simple counter contract: one `i64` that can be incremented, decremented and reset.

use near_sdk::{log, near};

/// The contract state: a single number, starting at `0`.
#[near(contract_state)]
#[derive(Default)]
pub struct Counter {
    num: i64,
}

#[near]
impl Counter {
    /// Initializes the contract with the number set to `0`.
    #[init]
    pub fn new() -> Self {
        Self { num: 0 }
    }

    /// Returns the current value.
    pub fn get_num(&self) -> i64 {
        self.num
    }

    /// Adds one to the current value and logs the new value.
    pub fn increment(&mut self) -> i64 {
        self.num = self
            .num
            .checked_add(1)
            .expect("Cannot increment past i64::MAX");
        log!("Increased number to {}", self.num);
        self.num
    }

    /// Subtracts one from the current value and logs the new value.
    pub fn decrement(&mut self) -> i64 {
        self.num = self
            .num
            .checked_sub(1)
            .expect("Cannot decrement below i64::MIN");
        log!("Decreased number to {}", self.num);
        self.num
    }

    /// Sets the value back to `0` and logs it.
    pub fn reset(&mut self) -> i64 {
        self.num = 0;
        log!("Reset number to 0");
        self.num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context() -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("alice.testnet".parse().unwrap())
            .build()
    }

    #[test]
    fn test_new_starts_at_zero() {
        testing_env!(get_context());
        let contract = Counter::new();
        assert_eq!(contract.get_num(), 0);
        assert!(get_logs().is_empty());
    }

    #[test]
    fn test_get_num() {
        testing_env!(get_context());
        let contract = Counter { num: 42 };
        assert_eq!(contract.get_num(), 42);
    }

    #[test]
    fn test_increment() {
        testing_env!(get_context());
        let mut contract = Counter::new();

        assert_eq!(contract.increment(), 1);
        assert_eq!(contract.get_num(), 1);
        assert_eq!(get_logs().last().unwrap(), "Increased number to 1");

        assert_eq!(contract.increment(), 2);
        assert_eq!(contract.get_num(), 2);
        assert_eq!(get_logs().last().unwrap(), "Increased number to 2");
    }

    #[test]
    fn test_decrement() {
        testing_env!(get_context());
        let mut contract = Counter { num: 1 };

        assert_eq!(contract.decrement(), 0);
        assert_eq!(contract.get_num(), 0);
        assert_eq!(get_logs().last().unwrap(), "Decreased number to 0");

        // Decrementing below zero is allowed.
        assert_eq!(contract.decrement(), -1);
        assert_eq!(contract.get_num(), -1);
        assert_eq!(get_logs().last().unwrap(), "Decreased number to -1");
    }

    #[test]
    fn test_reset() {
        testing_env!(get_context());
        let mut contract = Counter::new();

        contract.increment();
        contract.increment();
        contract.increment();
        assert_eq!(contract.get_num(), 3);

        assert_eq!(contract.reset(), 0);
        assert_eq!(contract.get_num(), 0);
        assert_eq!(get_logs().last().unwrap(), "Reset number to 0");
    }

    #[test]
    #[should_panic(expected = "Cannot increment past i64::MAX")]
    fn test_increment_overflow_panics() {
        testing_env!(get_context());
        let mut contract = Counter { num: i64::MAX };
        contract.increment();
    }

    #[test]
    #[should_panic(expected = "Cannot decrement below i64::MIN")]
    fn test_decrement_overflow_panics() {
        testing_env!(get_context());
        let mut contract = Counter { num: i64::MIN };
        contract.decrement();
    }
}
