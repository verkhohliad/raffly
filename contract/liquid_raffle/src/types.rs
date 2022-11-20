use crate::*;

#[ext_contract(ext_ft_contract)]
trait ExtFtContract {
    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        msg: String,
        memo: Option<String>,
    ) -> Promise;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LiquidUnstakeResult {
    pub near: U128String,
    pub fee: U128String,
    pub meta: U128String,
}

#[ext_contract(ext_meta_pool)]
pub trait ExtMetaPool {
    fn get_account_staked_balance(&self, account_id: AccountId) -> U128String;

    fn get_account_unstaked_balance(&self, account_id: AccountId) -> U128String;

    fn get_account_total_balance(&self, account_id: AccountId) -> U128String;

    fn deposit(&mut self);

    fn deposit_and_stake(&mut self);

    fn withdraw(&mut self, amount: U128String) -> Promise;
    fn withdraw_all(&mut self) -> Promise;
    fn withdraw_unstaked(&mut self) -> Promise;

    fn stake(&mut self, amount: U128String);

    fn unstake(&mut self, amount: U128String);

    fn unstake_all(&mut self);

    fn harvest_meta(&mut self) -> Promise;

    fn liquid_unstake(
        &mut self,
        st_near_to_burn: U128String,
        min_expected_near: U128String,
    ) -> LiquidUnstakeResult;

    fn get_st_near_price(&self) -> U128String;
}
