use crate::*;

#[ext_contract(this_raffle_factory)]
trait Callbacks {
    fn after_raffle_deployed(
        &mut self,
        #[callback_result] st_balance_result: Result<U128String, PromiseError>,
        st_price: u128,
    ) -> u128;
}

#[near_bindgen]
impl Callbacks for RaffleFactory {
    #[private]
    fn after_raffle_deployed(
        &mut self,
        #[callback_result] result: Result<U128String, PromiseError>,
        st_price: u128,
    ) {
        if result.is_err() {
            panic!("Raffle deployment has failed.");
        }

        log!("After deploy: {}", result.unwrap().0);

        // event!(
        //     r#"{{"event":"DELAYED_UNSTAKE","st_balance": "{}", "st_price": "{}", "unstake_amount": "{}"}}"#,
        //     st_balance,
        //     st_price,
        //     unstake_amount
        // );
        // log!(
        //     "Starting unstaking period: st_balance: {}, st_price: {}, unstake_amount: {}",
        //     st_balance,
        //     st_price,
        //     unstake_amount
        // );
    }
}
