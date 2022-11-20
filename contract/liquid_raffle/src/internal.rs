use crate::*;

#[near_bindgen]
impl LiquidRaffle {
    pub fn assert_owner_calling(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Can only be called by the owner");
    }

    // participating in the next raffle while current one is ongoing (rewards till the end will be part of winner prize)
    #[private]
    pub(crate) fn internal_pre_participate(&mut self, sender_id: AccountId, amount: u128) {
        self.prolonged_amount += amount;
        self.prolonged_assets.insert(
            &sender_id.clone(),
            &(self.prolonged_assets.get(&sender_id).unwrap_or(0) + &amount)
        );
    }

    #[private]
    pub(crate) fn internal_participate(&mut self, sender_id: AccountId, amount: u128, is_auto_prolong: bool) {
        let tickets_amount = (amount / self.ticket_price) as u64;

        for _ in 0..(tickets_amount as u64) {
            self.tickets.insert(&self.ticket_counter, &sender_id.clone());

            self.ticket_counter += 1;
        }

        self.locked_amount += amount;

        if is_auto_prolong {
            self.prolonged_amount += amount;
        }

        if is_auto_prolong {
            self.prolonged_assets.insert(
                &sender_id.clone(),
                &(self.prolonged_assets.get(&sender_id).unwrap_or(0) + &amount)
            );
        }

        self.locked_assets.insert(
            &sender_id.clone(),
            &(self.locked_assets.get(&sender_id).unwrap_or(0) + &amount)
        );

        event!(
            r#"{{"event":"PARTICIPATED","sender_id":"{}","amount":"{}","is_auto_prolong":"{}","tickets_amount:"{}"}}"#,
            sender_id,
            amount,
            is_auto_prolong,
            tickets_amount,
        );
        log!(
            "PARTICIPATED: sender_id={}, amount={}, is_auto_prolong={}, tickets_amount={}",
            sender_id,
            amount,
            is_auto_prolong,
            tickets_amount,
        );

        log!(
            "prolonged_assets={:?}, locked_assets={:?}, tickets={:?}, locked_amount={}, prolonged_amount={}, ticket_counter={}",
            self.prolonged_assets.to_vec(),
            self.locked_assets.to_vec(),
            self.tickets.to_vec(),
            self.locked_amount,
            self.prolonged_amount,
            self.ticket_counter
        );
    }

    #[private]
    pub(crate) fn internal_end_raffle(&mut self) {
        // prize = stacking rewards (st_price difference)
        // maybe cut decimals after 10-4 or less
        let prize = self.locked_amount / self.end_st_near_price * (self.end_st_near_price - self.start_st_near_price);

        let owner_commission = prize / 100 * self.owner_commission;
        let service_fee = prize / 100 * SERVICE_FEE;

        self.claimable_assets.insert(
            &self.raffle_owner,
            &(self.claimable_assets.get(&self.raffle_owner).unwrap_or(0) + owner_commission)
        );
        self.claimable_assets.insert(
            &env::current_account_id(),
            &(self.claimable_assets.get(&env::current_account_id()).unwrap_or(0) + service_fee)
        );

        let seed: [u8; 32] = env::random_seed().try_into().unwrap();
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let winner_ticket = rng.gen_range(0, self.ticket_counter).into();

        self.winner = self.tickets.get(&winner_ticket).unwrap();
        self.winner_ticket = winner_ticket;
        self.winner_amount = prize;

        self.is_raffle_on = false;
        self.end_time = env::block_timestamp();
        self.start_time = self.end_time + self.starting_period;
        self.round += 1;

        event!(
            r#"{{"event":"END","owner_commission":"{}","service_fee":"{}","prize":"{}","winner_ticket:"{}","winner":"{}","time":"{}","start_st_near_price":"{}","end_st_near_price":"{}","ticket_counter": "{}"}}"#,
            owner_commission,
            service_fee,
            prize,
            winner_ticket,
            self.winner,
            self.end_time,
            self.start_st_near_price,
            self.end_st_near_price,
            self.ticket_counter
        );
        log!(
            "END: owner_commission={}, service_fee={}, prize={}, winner_ticket={}, winner={}, time={}, start_st_near_price={}, end_st_near_price={}, ticket_counter={}",
            owner_commission,
            service_fee,
            prize,
            winner_ticket,
            self.winner,
            self.end_time,
            self.start_st_near_price,
            self.end_st_near_price,
            self.ticket_counter
        );

        // if assets not prolonged -> add to claimable entity to being able to been withdrawn
        self.locked_assets.iter().for_each(|(account_id, amount)| {
            if self.prolonged_assets.get(&account_id).is_none() {
                self.claimable_assets.insert(
                    &account_id.clone(),
                    &(self.claimable_assets.get(&account_id).unwrap_or(0) + amount)
                );
            }
        });

        // set prize to winner balance
        self.claimable_assets.insert(
            &self.winner,
            &(self.claimable_assets.get(&self.winner).unwrap_or(0) + (prize - owner_commission - service_fee))
        );
        // self.claimable_assets.get(&self.winner).unwrap_or(0) += prize;

        let storage_usage = env::storage_usage();

        log!(
            "claimable_assets={:?}, tickets={:?}, prolonged_assets={:?}, locked_assets={:?}, storage_usage={}",
            self.claimable_assets.to_vec(),
            self.tickets.to_vec(),
            self.prolonged_assets.to_vec(),
            self.locked_assets.to_vec(),
            storage_usage
        );

        self.internal_cleanup();

        // prefill prolonged assets for next raffle round tickets
        // TODO: resolve this hack with prolong_assets ownership delegation
        let mut prolonged_assets = mem::replace(
            &mut self.prolonged_assets,
            UnorderedMap::new(StorageKeys::ProlongedAssets)
        );

        for (account_id, amount) in prolonged_assets.iter() {
            self.internal_participate(account_id.clone(), amount.clone(), false);
        }

        self.prolonged_assets = prolonged_assets;
    }

    #[private]
    pub(crate) fn internal_cleanup(&mut self) {
        self.tickets.clear();
        self.locked_assets.clear();
        self.locked_amount = 0;
        self.start_st_near_price = 0;
        self.end_st_near_price = 0;
        self.ticket_counter = 0;

        log!(
            "cleanup, tickets={:?}",
            self.tickets.to_vec(),
        );
    }

    #[private]
    pub(crate) fn internal_claim(&mut self) -> u128 {
        match self.claimable_assets.get(&env::predecessor_account_id()) {
            None => panic!("You have no assets to claim"),
            Some(amount) => {
                event!(
                    r#"{{"event":"CLAIM","account_id":"{}","amount":"{}"}}"#,
                    env::predecessor_account_id(),
                    amount
                );
                log!(
                    "Claim: account_id: {}, amount: {}",
                    env::predecessor_account_id(),
                    amount
                );

                // handle promise success?
                Promise::new(env::predecessor_account_id()).transfer(amount);

                self.claimable_assets.remove(&env::predecessor_account_id());

                amount
            },
        }
    }

    #[private]
    pub(crate) fn internal_remove_prolongation(&mut self, account_id: AccountId) {
        let amount = self.prolonged_assets.get(&account_id).unwrap_or(0);

        event!(
            r#"{{"event":"REMOVE_PROLONGATION","account_id":"{}","amount":"{}"}}"#,
            account_id,
            amount
        );
        log!(
            "REMOVE_PROLONGATION: account_id: {}, amount: {}",
            account_id,
            amount
        );

        self.prolonged_amount -= amount;

        self.prolonged_assets.remove(&account_id);
    }

    #[private]
    pub(crate) fn internal_start_raffle(&mut self, st_price: u128) {
        event!(
            r#"{{"event":"START","time":"{}","st_price":"{}"}}"#,
            env::block_timestamp(),
            st_price
        );
        log!(
            "Start raffle at {}, st_price: {}",
            env::block_timestamp(),
            st_price
        );

        self.start_time = env::block_timestamp();
        self.end_time = self.start_time + self.ending_period;

        self.start_st_near_price = st_price;

        self.is_raffle_on = true;
    }
}
