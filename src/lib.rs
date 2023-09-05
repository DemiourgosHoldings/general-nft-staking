#![no_std]

use constants::{DEFAULT_UNBONDING_TIME_PENALTY, ERR_FAILED_UNBONDING, ERR_ONE_TOKEN_ID_SUPPORTED};
use staking_context::StakingContext;
use types::start_unbonding_payload::StartUnbondingPayload;

use crate::constants::ERR_NOTHING_TO_CLAIM;

multiversx_sc::imports!();

pub mod constants;
pub mod staking_context;
pub mod staking_modules;
pub mod storage;
pub mod types;

/// An lib contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait NftStakingContract:
    storage::config::ConfigModule
    + storage::score::ScoreStorageModule
    + storage::user_data::UserDataStorageModule
{
    #[init]
    fn init(&self) {
        self.unbonding_time_penalty()
            .set_if_empty(&DEFAULT_UNBONDING_TIME_PENALTY);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments = self.call_value().all_esdt_transfers();
        self.require_same_token_id(&payments);
        let context = StakingContext::new(self, &payments.get(0).token_identifier);
        context.add_to_stake(&payments);
    }

    #[endpoint(startUnbonding)]
    fn start_unbonding(&self, payload: StartUnbondingPayload<Self::Api>) {
        self.require_unbonding_is_valid(&payload);

        let context = StakingContext::new(self, &payload.token_identifier);
        let is_unbonding_successful = context.start_unbonding(payload);
        require!(is_unbonding_successful, ERR_FAILED_UNBONDING);
    }

    #[endpoint(claimUnbonded)]
    fn claim_unbonded(&self) {
        let caller = self.blockchain().get_caller();
        let block_timestamp = self.blockchain().get_block_timestamp();
        let unbonding_time_penalty = self.unbonding_time_penalty().get();

        let mut payments = ManagedVec::new();
        for unbonding_batch in self.unbonding_assets(&caller).iter() {
            let (start_unbonding_timestamp, unbonding_payload) = unbonding_batch;
            if start_unbonding_timestamp + unbonding_time_penalty > block_timestamp {
                continue;
            }

            let batch_payments = unbonding_payload.get_payments();
            payments.extend(&batch_payments);
        }

        require!(payments.len() > 0, ERR_NOTHING_TO_CLAIM);

        self.send().direct_multi(&caller, &payments);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {}

    fn require_same_token_id(&self, payments: &ManagedVec<EsdtTokenPayment>) {
        let token_id = payments.get(0).token_identifier.clone();
        let other_token_id_payment = payments.iter().find(|p| p.token_identifier != token_id);

        require!(other_token_id_payment.is_none(), ERR_ONE_TOKEN_ID_SUPPORTED);
    }

    fn require_unbonding_is_valid(&self, payload: &StartUnbondingPayload<Self::Api>) {
        require!(
            self.staked_nfts(&payload.token_identifier)
                .contains_key(&self.blockchain().get_caller())
                && !payload.is_empty(),
            ERR_FAILED_UNBONDING
        );
    }
}
