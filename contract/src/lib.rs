use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct contractA {
    greeting: String,
}
impl Default for contractA {
    fn default() -> Self {
        Self {
            greeting: String::from("Hello anh em"),
        }
    }
}
#[near_bindgen]
impl contractA {
    pub fn get_greeting(&self) -> String {
        log!("Toi la tran van chinh");
        return String::from("Mai Quynh");
    }
}
