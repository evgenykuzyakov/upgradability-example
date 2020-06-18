use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize)]
pub struct PrevStatusMessage {
    owner_id: AccountId,
    records: UnorderedMap<String, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    owner_id: AccountId,
    records: UnorderedMap<String, String>,
    num_records: UnorderedMap<String, u64>,
    last_record: String,
}

impl Default for StatusMessage {
    fn default() -> Self {
        env::panic(b"Not initialized");
    }
}

#[near_bindgen]
impl StatusMessage {
    #[init]
    pub fn migrate() -> Self {
        let old_state: PrevStatusMessage = env::state_read().expect("Not initialized");
        Self {
            owner_id: old_state.owner_id,
            records: old_state.records,
            num_records: UnorderedMap::new(b"n".to_vec()),
            last_record: String::new(),
        }
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            records: UnorderedMap::new(b"r".to_vec()),
            num_records: UnorderedMap::new(b"n".to_vec()),
            last_record: String::new(),
        }
    }

    pub fn set_status(&mut self, message: String) {
        let account_id = env::predecessor_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "bob_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::new("bob.near".to_string());
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::new("bob.near".to_string());
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
