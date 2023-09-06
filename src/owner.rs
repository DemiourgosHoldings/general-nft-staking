use crate::constants::{ERR_INVALID_REWARD_TOKEN_ID, ERR_REWARD_ALREADY_DISTRIBUTED};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait OwnerModule:
    crate::storage::config::ConfigModule + crate::storage::user_data::UserDataStorageModule
{
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
