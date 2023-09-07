use crate::{
    constants::{DEB_DENOMINATION, ERR_INVALID_REWARD_TOKEN_ID, ERR_REWARD_ALREADY_DISTRIBUTED},
    utils::secure_rewards,
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait OwnerModule:
    crate::storage::config::ConfigModule
    + crate::storage::user_data::UserDataStorageModule
    + crate::storage::score::ScoreStorageModule
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

    #[only_owner]
    #[endpoint(updateDeb)]
    fn update_deb(&self, user_address: ManagedAddress, new_deb: BigUint) {
        secure_rewards(self, &user_address);
        let deb_denomination = BigUint::from(DEB_DENOMINATION);
        let mut old_deb = self.user_deb(&user_address).get();
        if &old_deb < &deb_denomination {
            old_deb = deb_denomination.clone();
        }

        let old_user_score = self.aggregated_user_staking_score(&user_address).get();
        let old_general_aggregated_score = self.aggregated_staking_score().get();

        let score_without_deb = &(&old_user_score * &deb_denomination) / &old_deb;
        let new_user_score = &(&score_without_deb * &new_deb) / &deb_denomination;
        let new_general_aggregated_score =
            &old_general_aggregated_score - &old_user_score + &new_user_score;

        self.aggregated_user_staking_score(&user_address)
            .set(new_user_score);
        self.aggregated_staking_score()
            .set(&new_general_aggregated_score);
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