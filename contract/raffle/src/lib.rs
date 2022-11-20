use std::mem;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::borsh::maybestd::collections::{HashMap, HashSet};
use near_sdk::json_types::{Base58CryptoHash, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, is_promise_success, log, near_bindgen, serde_json,
    AccountId, Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault, PromiseOrValue,
    Timestamp, require, PromiseError, Promise, PromiseResult
};

pub const VIRTUAL_ACC: &str = "xxxxxxx.near";

#[macro_export]
macro_rules! event {
    ($($arg:tt)*) => ({
        env::log(format!($($arg)*).as_bytes());
    });
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKeys {
    Tickets,
    ClaimableAssets,
    LockedAssets,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Raffle {
    /// No-changeable
    pub owner: AccountId,           // contract owner (factory contract)
    pub raffle_owner: AccountId,   // owner of the raffle
    pub token_id: AccountId,       // token id

    /// Changeable
    pub owner_commission: u128,
    pub claimable_assets: UnorderedMap<AccountId, u128>,    // set after raffle is finished with prize
    pub ticket_price: u128,
    pub ending_period: u64,         // time from start to end
    pub is_raffle_on: bool,
    pub round: u64,             // round number
    pub raffle_name: String,    // name of raffle
    pub raffle_description: String, // description of raffle

    /// Changeable-cleanable(after raffle round is finished)
    pub tickets: UnorderedMap<u64, AccountId>,          // tickets
    pub locked_assets: UnorderedMap<AccountId, u128>,   // locked assets for current raffle round
    pub ticket_counter: u64,        // tickets counter
    pub locked_amount: u128,        // amount of locked assets for current raffle round
    pub start_time: u64,          // time when raffle started
    pub end_time: u64,            // time when raffle ended

    // clean on start
    pub winner: AccountId,
    pub winner_ticket: u64,
    pub winner_amount: u128,
}

impl Default for Raffle {
    fn default() -> Self{
        env::panic(b"The contract should be initialized before usage");
    }
}

#[near_bindgen]
impl Raffle {
    #[private]
    #[init(ignore_state)]
    pub fn new(
        raffle_owner: AccountId,
        ticket_price: U128,
        ending_period: U64,
        owner_commission: U128,
        raffle_name: String,
        raffle_description: String,
        token_id: AccountId,
    ) -> Self {
        let ticket_price = ticket_price.0;
        assert!(ticket_price > 0, "Ticket price should be greater than 0");
        assert!(ending_period.0 > 0, "Ending time should be more than 0");
        assert!(owner_commission.0 > 0, "Owner commission should be more than 0");
        assert!(!raffle_name.is_empty(), "Raffle name should not be empty");
        assert!(!raffle_description.is_empty(), "Raffle description should not be empty");
        // add check for token_id

        event!(
            r#"{{"event":"INIT","raffle_owner":"{}","ticket_price":"{}","ending_period:"{}","owner_commission":"{}","owner":"{}","raffle_name":"{}","raffle_description":"{}"}}"#,
            raffle_owner,
            ticket_price,
            ending_period.0,
            owner_commission.0,
            env::predecessor_account_id(),
            raffle_name,
            raffle_description
        );
        log!(
            "Init raffle raffle_owner:{},ticket_price:{},ending_period:{},owner_commission:{},owner:{},raffle_name:{},raffle_description:{}",
            raffle_owner,
            ticket_price,
            ending_period.0,
            owner_commission.0,
            env::predecessor_account_id(),
            raffle_name,
            raffle_description
        );

        Self {
            owner: env::predecessor_account_id(),
            raffle_owner,
            token_id,
            owner_commission: owner_commission.0,
            tickets: UnorderedMap::new(b"T1".to_vec()),
            claimable_assets: UnorderedMap::new(b"C1".to_vec()),
            locked_assets: UnorderedMap::new(b"L1".to_vec()),
            ticket_counter: 0,
            ticket_price,
            locked_amount: 0,
            ending_period: ending_period.0,
            start_time: 0,
            end_time: env::block_timestamp(),
            is_raffle_on: false,
            winner: VIRTUAL_ACC.parse().unwrap(),
            winner_ticket: 0,
            winner_amount: 0,
            round: 0,
            raffle_name,
            raffle_description,
        }
    }
}
