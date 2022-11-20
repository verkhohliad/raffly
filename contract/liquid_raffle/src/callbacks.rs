use crate::*;

#[ext_contract(this_raffle)]
trait Callbacks {
    fn after_ft_transfer(
        &mut self,
        prize: u128,
    ) -> u128;

    fn after_st_wrap(
        &mut self,
        sender_id: AccountId,
        amount: u128,
        is_auto_prolong: bool,
    ) -> u128;

    fn start_raffle_callback(
        &mut self,
        #[callback_result] result: Result<U128String, PromiseError>
    );

    fn end_raffle_callback(
        &mut self,
        #[callback_result] result: Result<(), PromiseError>
    );

    fn get_st_balance(&self, #[callback_result] st_price_result: Result<U128String, PromiseError>);
    fn unstake_callback(
        &mut self,
        #[callback_result] st_balance_result: Result<U128String, PromiseError>,
        st_price: u128,
    );
    fn after_unstake(&mut self, #[callback_result] result: Result<(), PromiseError>, st_price: u128);
}

#[near_bindgen]
impl Callbacks for LiquidRaffle {
    #[private]
    fn after_ft_transfer(
        &mut self,
        prize: u128,
    ) -> u128 {
        let promise_success = is_promise_success();

        if promise_success {
            self.internal_cleanup();
        } else {
            panic!("Token transfer has failed.");
        }

        prize.into()
    }

    #[private]
    fn after_st_wrap(&mut self, sender_id: AccountId, amount: u128, is_auto_prolong: bool) -> u128 {
        let promise_success = is_promise_success();

        if promise_success {
            if self.is_raffle_on && is_auto_prolong {
                self.internal_pre_participate(sender_id, amount);
            } else {
                self.internal_participate(sender_id, amount, is_auto_prolong);
            }
        } else {
            // refund
            Promise::new(sender_id).transfer(amount);
            log!("Wrapping has failed. Refunding {} NEAR", amount);
        }

        amount.into()
    }

    #[private]
    fn start_raffle_callback(&mut self, #[callback_result] result: Result<U128String, PromiseError>) {
        let promise_success = is_promise_success();

        if promise_success {
            let price = result.unwrap();

            self.internal_start_raffle(price.0);
        } else {
            panic!("Staking price has failed.");
        }
    }

    #[private]
    fn end_raffle_callback(&mut self, #[callback_result] result: Result<(), PromiseError>) {
        let promise_success = is_promise_success();

        if promise_success {
            self.internal_end_raffle();
        } else {
            panic!("Staking price has failed.");
        }
    }

    #[private]
    fn get_st_balance(&self, #[callback_result] st_price_result: Result<U128String, PromiseError>) {
        let promise_success = is_promise_success();

        if promise_success {
            let st_price = st_price_result.unwrap();

            ext_meta_pool::ext(META_POOL_ID.parse().unwrap())
                .get_account_staked_balance(env::current_account_id())
            .then(
                Self::ext(env::current_account_id())
                .unstake_callback(st_price.0.into())
            );
        } else {
            panic!("Getting price has failed.");
        }
    }

    #[private]
    fn unstake_callback(
        &mut self,
        #[callback_result] st_balance_result: Result<U128String, PromiseError>,
        st_price: u128,
    ) {
        if st_balance_result.is_err() {
            panic!("Getting stacking balance has failed.");
        }

        let st_balance = st_balance_result.unwrap().0;

        let unstake_amount = st_balance - &self.prolonged_amount;

        event!(
            r#"{{"event":"DELAYED_UNSTAKE","st_balance": "{}", "st_price": "{}", "unstake_amount": "{}"}}"#,
            st_balance,
            st_price,
            unstake_amount
        );
        log!(
            "Starting unstaking period: st_balance: {}, st_price: {}, unstake_amount: {}",
            st_balance,
            st_price,
            unstake_amount
        );

        ext_meta_pool::ext(META_POOL_ID.parse().unwrap())
            .unstake(near_sdk::json_types::U128(unstake_amount))
        .then(
            Self::ext(env::current_account_id())
                .after_unstake(st_price)
        );
    }

    #[private]
    fn after_unstake(&mut self, #[callback_result] result: Result<(), PromiseError>, st_price: u128) {
        if result.is_err() {
            panic!("Unstaking has failed.");
        }

        self.end_st_near_price = st_price;
        log!("Unstake period started successfully");
    }
}
