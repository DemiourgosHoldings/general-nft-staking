#![no_std]

use constants::{DEFAULT_UNBONDING_TIME_PENALTY, ERR_FAILED_UNBONDING, ERR_ONE_TOKEN_ID_SUPPORTED};
use staking_context::StakingContext;
use types::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload};
use utils::get_all_pending_rewards;

use crate::{
    constants::{ERR_INVALID_STAKED_TOKEN_ID, ERR_NOTHING_TO_CLAIM},
    utils::claim_all_pending_rewards,
};

multiversx_sc::imports!();

pub mod constants;
pub mod owner;
pub mod requirements;
pub mod staking_context;
pub mod staking_modules;
pub mod storage;
pub mod types;
pub mod utils;
pub mod views;

#[multiversx_sc::contract]
pub trait NftStakingContract:
    storage::config::ConfigModule
    + storage::score::ScoreStorageModule
    + storage::user_data::UserDataStorageModule
    + owner::OwnerModule
    + views::ViewsModule
    + requirements::RequirementsModule
{
    #[init]
    fn init(&self, primary_reward_token_identifier: TokenIdentifier) {
        self.unbonding_time_penalty()
            .set_if_empty(&DEFAULT_UNBONDING_TIME_PENALTY);
        self.primary_reward_token_identifier()
            .set_if_empty(&primary_reward_token_identifier);
        self.reward_token_identifiers()
            .insert(primary_reward_token_identifier);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments = self.call_value().all_esdt_transfers();
        self.require_same_token_id(&payments);
        let mut context = StakingContext::new(self, &payments.get(0).token_identifier);
        context.add_to_stake(&payments);
        context.drop();
    }

    #[endpoint(startUnbonding)]
    fn start_unbonding(&self, payload: StartUnbondingPayload<Self::Api>) {
        self.require_unbonding_is_valid(&payload);

        let mut context = StakingContext::new(self, &payload.token_identifier);
        let is_unbonding_successful = context.start_unbonding(payload);
        require!(is_unbonding_successful, ERR_FAILED_UNBONDING);
        context.drop();
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
    fn claim_rewards(&self) {
        let caller = &self.blockchain().get_caller();

        let pending_rewards = claim_all_pending_rewards(self, caller);
        require!(pending_rewards.len() > 0, ERR_NOTHING_TO_CLAIM);

        self.send().direct_multi(&caller, &pending_rewards);
    }

    #[view(getPendingReward)]
    fn get_pending_reward(&self, address: ManagedAddress) -> ManagedVec<EsdtTokenPayment> {
        let store_pending_rewards = false;
        get_all_pending_rewards(self, &address, store_pending_rewards)
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
                && !payload.is_empty()
                && !self.has_duplicate_nonces(&payload.items),
            ERR_FAILED_UNBONDING
        );
    }

    fn require_stake_token_identifier_can_be_staked(&self, token_identifier: &TokenIdentifier) {
        require!(
            self.eligible_stake_token_identifiers()
                .contains(token_identifier),
            ERR_INVALID_STAKED_TOKEN_ID
        );
    }

    fn has_duplicate_nonces(&self, items: &ManagedVec<NonceQtyPair<Self::Api>>) -> bool {
        let mut nonces: ManagedVec<u64> = items.iter().map(|item| item.nonce).collect();
        nonces.sort_unstable();
        for i in 0..(nonces.len() - 1) {
            if nonces.get(i) == nonces.get(i + 1) {
                return true;
            }
        }
        false
    }
}
