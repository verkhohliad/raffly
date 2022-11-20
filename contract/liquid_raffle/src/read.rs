use crate::*;

#[derive(Serialize, BorshDeserialize, BorshSerialize)]
pub struct State {
    /// No-changeable
    pub owner: AccountId,           // contract owner (factory contract)
    pub raffle_owner: AccountId,   // owner of the raffle
    pub liquid: bool,            // is liquid raffle

    /// Changeable
    pub owner_commission: u128,
    pub prolonged_assets: Vec<(AccountId, u128)>,    // auto pre filling tickets for next raffle round
    pub claimable_assets: Vec<(AccountId, u128)>,    // set after raffle is finished with prize
    pub ticket_price: U128,
    pub starting_period: u64,       // time from setup to start (restart)
    pub ending_period: u64,         // time from start to end
    pub is_raffle_on: bool,
    pub prolonged_amount: U128, // assets that are prolonged
    pub round: u64,             // round number
    pub raffle_name: String,    // name of raffle
    pub raffle_description: String, // description of raffle

    /// Changeable-cleanable(after raffle round is finished)
    pub tickets: Vec<(u64, AccountId)>,          // tickets
    pub locked_assets: Vec<(AccountId, u128)>,   // locked assets for current raffle round
    pub ticket_counter: u64,        // tickets counter
    pub locked_amount: U128,        // amount of locked assets for current raffle round
    pub start_time: u64,          // time when raffle started
    pub end_time: u64,            // time when raffle ended
    pub start_st_near_price: U128,      // price of stacked token in near on moment of start lottery
    pub end_st_near_price: U128,        // price of stacked token in near on moment of end lottery

    // clean on start
    pub winner: AccountId,
    pub winner_ticket: u64,
    pub winner_amount: U128,
}

#[near_bindgen]
impl LiquidRaffle {
    pub fn get_state(&self) -> State {
        let state: Self = env::state_read().expect("failed");

        State{
            owner: state.owner,
            raffle_owner: state.raffle_owner,
            liquid: state.liquid,
            owner_commission: DEFAULT_OWNER_COMMISSION,
            tickets: state.tickets.to_vec(),
            prolonged_assets: state.prolonged_assets.to_vec(),
            claimable_assets: state.claimable_assets.to_vec(),
            locked_assets: state.locked_assets.to_vec(),
            ticket_counter: state.ticket_counter,
            ticket_price: U128(state.ticket_price),
            locked_amount: U128(state.locked_amount),
            prolonged_amount: U128(state.prolonged_amount),
            starting_period: state.starting_period,
            ending_period: state.ending_period,
            start_time: state.start_time,
            end_time: state.end_time,
            is_raffle_on: state.is_raffle_on,
            winner: state.winner,
            winner_ticket: state.winner_ticket,
            winner_amount: U128(state.winner_amount),
            start_st_near_price: U128(state.start_st_near_price),
            end_st_near_price: U128(state.end_st_near_price),
            round: state.round,
            raffle_name: state.raffle_name,
            raffle_description: state.raffle_description,
        }
    }
}
