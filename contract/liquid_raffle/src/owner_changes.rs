use crate::*;

#[near_bindgen]
impl LiquidRaffle {
    #[private]
    pub fn change_starting_period(&mut self, new_starting_period: U64) {
        self.assert_owner_calling();
        assert!(new_starting_period.0 > 0, "Ending time should be more than 0");

        event!(
            r#"{{"event":"CHANGE_STARTING_PERIOD","new_starting_period":"{}","old_starting_period":"{}"}}"#,
            new_starting_period.0,
            self.starting_period
        );
        log!(
            "CHANGE_ENDING_PERIOD: new_starting_period: {}, old_starting_period: {}",
            new_starting_period.0,
            self.starting_period
        );

        self.starting_period = new_starting_period.0;
    }

    #[private]
    pub fn change_ending_period(&mut self, new_ending_period: U64) {
        self.assert_owner_calling();
        assert!(new_ending_period.0 > 0, "Ending time should be more than 0");

        event!(
            r#"{{"event":"CHANGE_ENDING_PERIOD","new_ending_period":"{}","old_ending_period":"{}"}}"#,
            new_ending_period.0,
            self.ending_period
        );
        log!(
            "CHANGE_ENDING_PERIOD: new_ending_period: {}, old_ending_period: {}",
            new_ending_period.0,
            self.ending_period
        );

        self.ending_period = new_ending_period.0;
    }

    #[private]
    pub fn change_raffle_name(&mut self, new_raffle_name: String) {
        self.assert_owner_calling();
        assert!(!new_raffle_name.is_empty(), "Raffle name should not be empty");

        event!(
            r#"{{"event":"CHANGE_NAME","new_raffle_name":"{}","old_raffle_name":"{}"}}"#,
            new_raffle_name,
            self.raffle_name
        );
        log!(
            "CHANGE_NAME: new_raffle_name: {}, old_raffle_name: {}",
            new_raffle_name,
            self.raffle_name
        );

        self.raffle_name = new_raffle_name;
    }

    #[private]
    pub fn change_raffle_description(&mut self, new_raffle_description: String) {
        self.assert_owner_calling();
        assert!(!new_raffle_description.is_empty(), "Raffle description should not be empty");

        event!(
            r#"{{"event":"CHANGE_DESCRIPTION","new_raffle_description":"{}","old_raffle_description":"{}"}}"#,
            new_raffle_description,
            self.raffle_description
        );
        log!(
            "CHANGE_DESCRIPTION: new_raffle_description: {}, old_raffle_description: {}",
            new_raffle_description,
            self.raffle_description
        );

        self.raffle_description = new_raffle_description;
    }
}
