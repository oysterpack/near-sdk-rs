use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, log, near_bindgen, Balance, Gas, Promise};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PromiseBob {}

const NO_DEPOSIT: Balance = 0;

const BASIC_GAS: Gas = 5_000_000_000_000;

const DAVE: &str = "c.place.meta";

fn log_it(s: &str) {
    log!(
        "#{}   I'm @{}. Called by @{}. {}",
        env::block_index(),
        env::current_account_id(),
        env::predecessor_account_id(),
        s
    );
}

#[ext_contract(ext_dave)]
pub trait Dave {
    fn get_data(&self) -> String;
}

#[near_bindgen]
impl PromiseBob {
    pub fn get_data(&self) -> Promise {
        log_it("get_data");
        ext_dave::get_data(&DAVE, NO_DEPOSIT, BASIC_GAS)
    }
}