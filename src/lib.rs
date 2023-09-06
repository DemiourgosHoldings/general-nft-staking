#![no_std]

use constants::{DEFAULT_UNBONDING_TIME_PENALTY, ERR_FAILED_UNBONDING, ERR_ONE_TOKEN_ID_SUPPORTED};
use staking_context::StakingContext;
use types::start_unbonding_payload::StartUnbondingPayload;
use utils::get_unstored_pending_rewards;

use crate::constants::{
    ERR_INVALID_REWARD_TOKEN_ID, ERR_NOTHING_TO_CLAIM, ERR_REWARD_ALREADY_DISTRIBUTED,
};

multiversx_sc::imports!();

pub mod constants;
pub mod staking_context;
pub mod staking_modules;
pub mod storage;
pub mod types;
pub mod utils;

/// An lib contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait NftStakingContract:
    storage::config::ConfigModule
    + storage::score::ScoreStorageModule
    + storage::user_data::UserDataStorageModule
{
    #[init]
    fn init(&self, reward_token_identifier: TokenIdentifier) {
        self.unbonding_time_penalty()
            .set_if_empty(&DEFAULT_UNBONDING_TIME_PENALTY);
        self.reward_token_identifier()
            .set_if_empty(&reward_token_identifier);
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

    #[view(getPendingReward)]
    fn get_pending_reward(&self, address: ManagedAddress) -> BigUint {
        let not_stored_rewards = get_unstored_pending_rewards(self, &address);
        let stored_rewards = match self.pending_rewards(&address).is_empty() {
            false => self.pending_rewards(&address).get(),
            true => BigUint::zero(),
        };

        not_stored_rewards + stored_rewards
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(distributeGeneralReward)]
    fn distribute_reward(&self) {
        let total_score = self.aggregated_staking_score().get();
        let payment = self.call_value().single_esdt();

        self.require_token_is_reward_token(&payment.token_identifier);

        let block_epoch = self.blockchain().get_block_epoch();
        let block_timestamp = self.blockchain().get_block_timestamp();

        self.require_reward_not_distributed(block_epoch);

        let reward_rate = payment.amount / total_score;

        self.reward_rate(block_epoch).set(reward_rate);
        self.reward_distribution_timestamp(block_epoch)
            .set(&block_timestamp);
    }

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

    fn require_reward_not_distributed(&self, epoch: u64) {
        require!(
            self.reward_distribution_timestamp(epoch).is_empty()
                && self.reward_rate(epoch).is_empty(),
            ERR_REWARD_ALREADY_DISTRIBUTED
        );
    }

    fn require_token_is_reward_token(&self, incoming_token_identifier: &TokenIdentifier) {
        require!(
            &self.reward_token_identifier().get() == incoming_token_identifier,
            ERR_INVALID_REWARD_TOKEN_ID
        );
    }
}
