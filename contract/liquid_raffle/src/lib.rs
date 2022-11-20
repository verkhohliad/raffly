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

mod callbacks;
mod internal;
mod types;
mod read;
mod migration;
mod owner_changes;
use crate::types::*;

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


#[macro_export]
macro_rules! event {
    ($($arg:tt)*) => ({
        env::log(format!($($arg)*).as_bytes());
    });
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKeys {
    Tickets,
    ProlongedAssets,
    ClaimableAssets,
    LockedAssets,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LiquidRaffle {
    /// No-changeable
    pub owner: AccountId,           // contract owner (factory contract)
    pub raffle_owner: AccountId,   // owner of the raffle
    pub liquid: bool,            // is liquid raffle

    /// Changeable
    pub owner_commission: u128,
    pub prolonged_assets: UnorderedMap<AccountId, u128>,    // auto pre filling tickets for next raffle round
    pub claimable_assets: UnorderedMap<AccountId, u128>,    // set after raffle is finished with prize
    pub ticket_price: u128,
    pub starting_period: u64,       // time from setup to start (restart)
    pub ending_period: u64,         // time from start to end
    pub is_raffle_on: bool,
    pub prolonged_amount: u128, // assets that are prolonged
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
    pub start_st_near_price: u128,      // price of stacked token in near on moment of start lottery
    pub end_st_near_price: u128,        // price of stacked token in near on moment of end lottery

    // clean on start
    pub winner: AccountId,
    pub winner_ticket: u64,
    pub winner_amount: u128,
}

/*
Flow:
1. Owner initializes the raffle [init]
2. Participants buy tickets during start period (could be auto-prolonged) [ft_transfer_call(for token contract), ft_on_transfer, participate, meta_pool_deposit, ft_on_transfer]
3. Heartbeat starts the raffle right after starting period ends [start_raffle]
4. Raffle processing...
5.0 Croncat calls start_unstake_period before raffle ending time (2-5 days)
5. Heartbeat stops the raffle with deciding winner, next raffle start time begin [end_raffle, meta_pool.harvest_meta] (calculate prize, handle users balances, staring period starts, cleanup)
6. Participants can withdraw their tickets, winner can withdraw his prize, owner can withdraw his commission (whenever they want) [meta_pool.liquid_unstake on demand]
7. Auto-prolonged assets goes to new raffle
8. New participants can buy tickets
9. Repeat from 3.

Details of raffle could be changed in any time, but will applied to the next iteration.
 */

impl Default for LiquidRaffle {
    fn default() -> Self{
        env::panic(b"The contract should be initialized before usage");
    }
}

#[near_bindgen]
impl LiquidRaffle {
    #[private]
    #[init(ignore_state)]
    pub fn new(
        raffle_owner: AccountId,
        ticket_price: U128,
        starting_period: U64,
        ending_period: U64,
        owner_commission: U128,
        raffle_name: String,
        raffle_description: String,
    ) -> Self {
        let ticket_price = ticket_price.0;
        assert!(ticket_price > 0, "Ticket price should be greater than 0");
        assert!(starting_period.0 > 0, "Starting period should be more than 0");
        assert!(ending_period.0 > 0, "Ending time should be more than 0");
        assert!(owner_commission.0 > 0, "Owner commission should be more than 0");
        assert!(!raffle_name.is_empty(), "Raffle name should not be empty");
        assert!(!raffle_description.is_empty(), "Raffle description should not be empty");

        event!(
            r#"{{"event":"INIT","raffle_owner":"{}","ticket_price":"{}","starting_period":"{}","ending_period:"{}","owner_commission":"{}","owner":"{}","raffle_name":"{}","raffle_description":"{}"}}"#,
            raffle_owner,
            ticket_price,
            starting_period.0,
            ending_period.0,
            owner_commission.0,
            env::predecessor_account_id(),
            raffle_name,
            raffle_description
        );
        log!(
            "Init raffle raffle_owner:{},ticket_price:{},starting_period:{},ending_period:{},owner_commission:{},owner:{},raffle_name:{},raffle_description:{}",
            raffle_owner,
            ticket_price,
            starting_period.0,
            ending_period.0,
            owner_commission.0,
            env::predecessor_account_id(),
            raffle_name,
            raffle_description
        );

        Self{
            owner: env::predecessor_account_id(),
            raffle_owner,
            liquid: true,
            owner_commission: owner_commission.0,
            tickets: UnorderedMap::new(b"T1".to_vec()),
            prolonged_assets: UnorderedMap::new(b"P1".to_vec()),
            claimable_assets: UnorderedMap::new(b"C1".to_vec()),
            locked_assets: UnorderedMap::new(b"L1".to_vec()),
            ticket_counter: 0,
            ticket_price,
            locked_amount: 0,
            prolonged_amount: 0,
            starting_period: starting_period.0,
            ending_period: ending_period.0,
            start_time: 0,
            end_time: env::block_timestamp(),
            is_raffle_on: false,
            winner: VIRTUAL_ACC.parse().unwrap(),
            winner_ticket: 0,
            winner_amount: 0,
            start_st_near_price: 0,
            end_st_near_price: 0,
            round: 0,
            raffle_name,
            raffle_description,
        }
    }

    // calling by cron.cat
    #[private]
    pub fn start_raffle(&mut self) {
        self.assert_owner_calling();
        assert!(!self.is_raffle_on, "Raffle is already on");
        assert!(self.end_time + self.starting_period < env::block_timestamp(), "Starting period did not finished");

        event!(r#"{{"event":"STARTING"}}"#);
        log!("Starting raffle");

        ext_meta_pool::ext(META_POOL_ID.parse().unwrap())
            .get_st_near_price()
            .then(
                Self::ext(env::current_account_id())
                    .start_raffle_callback()
            );
    }

    // calling by cron.cat, should happen by 4 epochs before end_raffle
    #[private]
    pub fn start_unstake_period(&self) {
        self.assert_owner_calling();
        assert!(self.is_raffle_on, "Raffle is not on");

        event!(r#"{{"event":"UNSTACKING"}}"#);
        log!("Starting unstake period");

        // fetch price - get_st_near_price
        // fetch stacked balance - get_account_staked_balance
        // unstake stacked - unstake(balance - prolonged_amount*price)

        ext_meta_pool::ext(META_POOL_ID.parse().unwrap())
            .get_st_near_price()
            .then(
                Self::ext(env::current_account_id())
                    .get_st_balance()
            );
    }

    // calling by cron.cat
    #[private]
    pub fn end_raffle(&mut self) {
        self.assert_owner_calling();
        assert!(self.start_time + self.ending_period < env::block_timestamp(), "Raffle cannot be ended yet");

        event!(r#"{{"event":"ENDING"}}"#);
        log!("Ending raffle");

        ext_meta_pool::ext(META_POOL_ID.parse().unwrap())
            .withdraw_unstaked()
            .then(
                Self::ext(env::current_account_id())
                    .end_raffle_callback()
            );
    }

    pub fn claim(&mut self) -> u128 {
        self.internal_claim()
    }

    pub fn remove_prolongation(&mut self) {
        self.internal_remove_prolongation(env::predecessor_account_id());
    }

    // set tickets, buy stacked assets
    #[payable]
    pub fn participate(&mut self, is_auto_prolong: bool) {
        // leave ONE_MILLI_NEAR of each ticket for storage
        let storage_fee = env::attached_deposit() / self.ticket_price * ONE_MILLI_NEAR;
        let amount = env::attached_deposit() - storage_fee;
        let sender_id = env::predecessor_account_id();

        assert!(amount > 0, "Amount should be greater than 0");
        assert!(!self.is_raffle_on || is_auto_prolong, "You cannot participate in the ongoing raffle without prolongation");
        assert_eq!(amount % self.ticket_price, 0, "Amount should be a multiple of ticket price");

        event!(
            r#"{{"event":"Participating", "amount":"{}","storage_fee":"{}"}}"#,
            env::attached_deposit(),
            storage_fee
        );
        log!(
            "Participating: amount:{},storage_fee:{}",
            env::attached_deposit(),
            storage_fee
        );

        // wrap asset to stAsset
        ext_meta_pool::ext(META_POOL_ID.parse().unwrap())
            .with_attached_deposit(amount)
            .deposit_and_stake()
            .then(
                Self::ext(env::current_account_id())
                    .after_st_wrap(sender_id, amount, is_auto_prolong)
            );
    }
}

