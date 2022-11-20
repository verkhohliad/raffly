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

const RAFFLE_CODE: &[u8] = include_bytes!("../../raffle/res/raffle.wasm");

pub type U128String = U128;

pub const VIRTUAL_ACC: &str = "xxxxxxx.near";

pub const META_POOL_ID: &str = "meta-pool.near";
pub const META_TOKEN_ID: &str = "meta-token.near";
// pub const META_POOL_ID: &str = "meta-v2.pool.testnet";
// pub const META_TOKEN_ID: &str = "token.meta.pool.testnet";

const GAS_FOR_FT_TRANSFER: Gas = near_sdk::Gas(15_000_000_000_000);
const GAS_FOR_AFTER_FT_TRANSFER: Gas = near_sdk::Gas(20_000_000_000_000);

/// useful constants
pub const ONE_E24: u128 = 1_000_000_000_000_000_000_000_000;
pub const NEAR: u128 = ONE_E24;
pub const ONE_NEAR: u128 = NEAR;
pub const NEAR_CENT: u128 = NEAR / 100;
pub const ONE_MILLI_NEAR: u128 = NEAR / 1_000;
pub const ONE_MICRO_NEAR: u128 = NEAR / 1_000_000;
pub const TWO_NEAR: u128 = 2 * NEAR;
pub const FIVE_NEAR: u128 = 5 * NEAR;
pub const TEN_NEAR: u128 = 10 * NEAR;
pub const K_NEAR: u128 = 1_000 * NEAR;

const ONE_YOCTO: Balance = 1;
const NO_DEPOSIT: Balance = 0;

const SERVICE_FEE: u128 = 1;
const DEFAULT_OWNER_COMMISSION: u128 = 4;

mod callbacks;

#[macro_export]
macro_rules! event {
    ($($arg:tt)*) => ({
        env::log(format!($($arg)*).as_bytes());
    });
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKeys {
    Raffles
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RaffleFactory {
    pub owner: AccountId,
    pub raffles: Vector<String>,    // addresses of created raffles
}

impl Default for RaffleFactory {
    fn default() -> Self{
        env::panic(b"The contract should be initialized before usage");
    }
}

// liquid raffle putted in manually, because it's already deployed and it gonna be only one for now
// so create raffle only will work for usual one and in the future for the custom ones
#[near_bindgen]
impl RaffleFactory {
    #[private]
    #[init(ignore_state)]
    pub fn new() -> Self {
        event!(r#"{{"event":"INIT"}}"#);
        log!("Init raffle factory");

        Self{
            owner: env::predecessor_account_id(),
            raffles: Vector::new(StorageKeys::Raffles),
        }
    }

    pub fn assert_owner_calling(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Can only be called by the owner");
    }

    pub fn get_raffles(&self) -> Vec<String> {
        self.raffles.to_vec()
    }

    pub fn create_raffle(
        &mut self,
        prefix: String,
        raffle_owner: AccountId,
        ticket_price: U128,
        ending_period: U64,
        owner_commission: U128,
        raffle_name: String,
        raffle_description: String,
        token_id: AccountId,
    ) -> Promise {
        self.assert_owner_calling();

        assert!(prefix.len() > 0, "Prefix should be not empty");
        assert!(ticket_price.0 > 0, "Ticket price should be more than 0");
        assert!(ending_period.0 > 0, "Ending period should be more than 0");
        assert!(owner_commission.0 > 0, "Owner commission should be more than 0");
        assert!(raffle_name.len() > 0, "Raffle name should be not empty");
        assert!(raffle_description.len() > 0, "Raffle description should be not empty");

        let sub_account_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );

        Promise::new(sub_account_id)
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(3 * NEAR)
            .deploy_contract(RAFFLE_CODE.to_vec())
            .then(
                Self::ext(env::current_account_id())
                    .after_raffle_deployed()
            )
    }
}
