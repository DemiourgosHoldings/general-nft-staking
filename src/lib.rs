#![no_std]

use staking_context::StakingContext;
use types::start_unbonding_payload::StartUnbondingPayload;

multiversx_sc::imports!();

pub mod constants;
pub mod staking_context;
pub mod staking_modules;
pub mod storage;
pub mod types;

/// An lib contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait NftStakingContract:
    storage::config::ConfigModule + storage::score::ScoreStorageModule
{
    #[init]
    fn init(&self) {}

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
        let context = StakingContext::new(self, &payload.token_identifier);
        context.start_unbonding(payload);
    }

    #[endpoint(claimUnbonded)]
    fn claim_unbonded(&self) {}

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {}

    fn require_same_token_id(&self, payments: &ManagedVec<EsdtTokenPayment>) {
        let token_id = payments.get(0).token_identifier.clone();
        let other_token_id_payment = payments.iter().find(|p| p.token_identifier != token_id);

        require!(
            other_token_id_payment.is_none(),
            "Only one token id is allowed per TX"
        );
    }
}
