use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldStruct {
    /// No-changeable
    pub owner: AccountId,           // contract owner (factory contract)
    pub raffle_owner: AccountId,   // owner of the raffle

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

#[near_bindgen]
impl LiquidRaffle {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldStruct = env::state_read().expect("failed");

        event!(r#"{{"event":"MIGRATION"}}"#);
        log!("Migration is happening!");

        Self{
            owner: old_state.owner,
            raffle_owner: old_state.raffle_owner,
            liquid: true,
            owner_commission: old_state.owner_commission,
            tickets: old_state.tickets,
            prolonged_assets: old_state.prolonged_assets,
            claimable_assets: old_state.claimable_assets,
            locked_assets: old_state.locked_assets,
            ticket_counter: old_state.ticket_counter,
            ticket_price: old_state.ticket_price,
            locked_amount: old_state.locked_amount,
            prolonged_amount: old_state.prolonged_amount,
            starting_period: old_state.starting_period,
            ending_period: old_state.ending_period,
            start_time: old_state.start_time,
            end_time: old_state.end_time,
            is_raffle_on: old_state.is_raffle_on,
            winner: old_state.winner,
            winner_ticket: old_state.winner_ticket,
            winner_amount: old_state.winner_amount,
            start_st_near_price: old_state.start_st_near_price,
            end_st_near_price: old_state.end_st_near_price,
            round: old_state.round,
            raffle_name: old_state.raffle_name,
            raffle_description: old_state.raffle_description,
        }
    }
}
