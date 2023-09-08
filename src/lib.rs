#![no_std]

use constants::{DEFAULT_UNBONDING_TIME_PENALTY, ERR_FAILED_UNBONDING, ERR_ONE_TOKEN_ID_SUPPORTED};
use staking_context::StakingContext;
use types::start_unbonding_payload::StartUnbondingPayload;
use utils::{get_unstored_pending_rewards, secure_rewards};

use crate::constants::ERR_NOTHING_TO_CLAIM;

multiversx_sc::imports!();

pub mod constants;
pub mod owner;
pub mod staking_context;
pub mod staking_modules;
pub mod storage;
pub mod types;
pub mod utils;

#[multiversx_sc::contract]
pub trait NftStakingContract:
    storage::config::ConfigModule
    + storage::score::ScoreStorageModule
    + storage::user_data::UserDataStorageModule
    + owner::OwnerModule
{
    #[init]
    fn init(&self, primary_reward_token_identifier: TokenIdentifier) {
        self.unbonding_time_penalty()
            .set_if_empty(&DEFAULT_UNBONDING_TIME_PENALTY);
        self.primary_reward_token_identifier()
            .set_if_empty(&primary_reward_token_identifier);
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
    fn claim_rewards(&self) {
        let caller = &self.blockchain().get_caller();

        let pending_rewards = self.claim_all_pending_rewards(caller);
        require!(pending_rewards.len() > 0, ERR_NOTHING_TO_CLAIM);

        self.send().direct_multi(&caller, &pending_rewards);
    }

    #[view(getPendingReward)]
    fn get_pending_reward(&self, address: ManagedAddress) -> BigUint {
        let primary_reward_token_id = self.primary_reward_token_identifier().get();
        let not_stored_rewards =
            get_unstored_pending_rewards(self, &address, &primary_reward_token_id);
        let stored_rewards = match self
            .pending_rewards(&address, &primary_reward_token_id)
            .is_empty()
        {
            false => self
                .pending_rewards(&address, &primary_reward_token_id)
                .get(),
            true => BigUint::zero(),
        };

        not_stored_rewards + stored_rewards
    }

    fn claim_all_pending_rewards(&self, caller: &ManagedAddress) -> ManagedVec<EsdtTokenPayment> {
        let base_reward_opt = self.claim_single_token_pending_rewards(
            caller,
            self.primary_reward_token_identifier().get(),
        );
        let mut pending_rewards = match base_reward_opt {
            Some(base_reward) => ManagedVec::from_single_item(base_reward),
            None => ManagedVec::new(),
        };
        for token_id in self.secondary_reward_token_identifiers().iter() {
            let reward_opt = self.claim_single_token_pending_rewards(caller, token_id);
            if reward_opt.is_none() {
                continue;
            }
            pending_rewards.push(reward_opt.unwrap());
        }

        pending_rewards
    }

    fn claim_single_token_pending_rewards(
        &self,
        caller: &ManagedAddress,
        token_identifier: TokenIdentifier,
    ) -> Option<EsdtTokenPayment> {
        secure_rewards(self, caller, &token_identifier);

        let pending_reward = self.get_pending_reward(caller.clone());
        if &pending_reward == &BigUint::zero() {
            return Option::None;
        }

        self.pending_rewards(&caller, &token_identifier).clear();
        let payment = EsdtTokenPayment::new(token_identifier, 0, pending_reward);

        Option::Some(payment)
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
}
