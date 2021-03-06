use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen};

const PUZZLE_NUMBER: u8 = 1;
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    puzzle_solution: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            puzzle_solution: solution,
        }
    }

    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn get_solution(&self) -> String {
        self.puzzle_solution.clone()
    }

    pub fn set_solution(&mut self, solution: String) {
        self.puzzle_solution = solution;
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input_string = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input_string);
        println!("{}, {}", self.puzzle_solution, hashed_input_hex);
        if self.puzzle_solution == hashed_input_hex {
            env::log_str("Right!");
            true
        } else {
            env::log_str("Try again.");
            false
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());
        
        let debug_solution = "I want to learn near";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    fn check_guess_solution() {
        let alice = AccountId::new_unchecked("puzzle.votanlean.testnet".to_string());
        let context = get_context(alice);
        testing_env!(context.build());

        let mut contract = Contract::new("b78c53475c4892592a74c88b928d77771b115daf6bb41d5350f9cf08cbb5c31a".to_string());
        let mut guess_result = contract.guess_solution("wrong answer here".to_string());
        assert!(!guess_result, "Expected wrong answer");
        assert_eq!(get_logs(), ["Try again."], "Expect fail log");
        guess_result = contract.guess_solution("I want to learn near".to_string());
        assert!(guess_result, "Expect right answer!");
        assert_eq!(get_logs(), ["Try again.", "Right!"], "Expect right after fail")
    }
}
