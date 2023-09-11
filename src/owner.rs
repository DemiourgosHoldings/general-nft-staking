use crate::{
    constants::{DEB_DENOMINATION, ERR_INVALID_REWARD_TOKEN_ID, ERR_REWARD_ALREADY_DISTRIBUTED},
    staking_modules::staking_module_type::StakingModuleType,
    utils::{apply_new_deb, secure_rewards},
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
        let total_score = self.aggregated_staking_score(&StakingModuleType::All).get();
        let payment = self.call_value().single_esdt();

        self.distribute_reward_handler(payment, total_score);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(distributeSecondaryReward)]
    fn distribute_secondary_reward(&self, target: TokenIdentifier) {
        let staking_module_type = self.stake_pool_type_configuration(&target).get();
        let total_score = self.aggregated_staking_score(&staking_module_type).get();
        let payment = self.call_value().single_esdt();

        self.distribute_reward_handler(payment, total_score);
    }

    #[only_owner]
    #[endpoint(updateDeb)]
    fn update_deb(&self, user_address: ManagedAddress, new_deb: BigUint) {
        for reward_token_id in self.reward_token_identifiers().iter() {
            self.update_deb_handler(&user_address, &new_deb, &reward_token_id);
        }
    }

    fn update_deb_handler(
        &self,
        user_address: &ManagedAddress,
        new_deb: &BigUint,
        token_identifier: &TokenIdentifier,
    ) {
        let staking_module_type = self.stake_pool_type_configuration(token_identifier).get();
        secure_rewards(self, &user_address, token_identifier, &staking_module_type);
        let deb_denomination = BigUint::from(DEB_DENOMINATION);
        let mut old_deb = self.user_deb(user_address).get();
        if &old_deb < &deb_denomination {
            old_deb = deb_denomination.clone();
        }
        self.update_score_handler(
            &staking_module_type,
            user_address,
            new_deb,
            &old_deb,
            &deb_denomination,
        );
    }

    fn update_score_handler(
        &self,
        staking_module_type: &StakingModuleType,
        user_address: &ManagedAddress,
        new_deb: &BigUint,
        old_deb: &BigUint,
        deb_denomination: &BigUint,
    ) {
        let old_user_score = self
            .aggregated_user_staking_score(staking_module_type, user_address)
            .get();
        let old_general_aggregated_score = self.aggregated_staking_score(staking_module_type).get();

        let (new_user_score, new_general_aggregated_score) = apply_new_deb::<Self>(
            &old_general_aggregated_score,
            &old_user_score,
            old_deb,
            new_deb,
            deb_denomination,
        );

        self.aggregated_user_staking_score(staking_module_type, user_address)
            .set(new_user_score);
        self.aggregated_staking_score(staking_module_type)
            .set(&new_general_aggregated_score);
    }

    fn distribute_reward_handler(&self, payment: EsdtTokenPayment, total_score: BigUint) {
        self.require_token_is_reward_token(&payment.token_identifier);
        let block_epoch = self.blockchain().get_block_epoch();
        let block_timestamp = self.blockchain().get_block_timestamp();

        self.require_reward_not_distributed(block_epoch, &payment.token_identifier);
        let reward_rate = payment.amount / total_score;

        self.reward_rate(block_epoch, &payment.token_identifier)
            .set(reward_rate);
        self.reward_distribution_timestamp(block_epoch, &payment.token_identifier)
            .set(&block_timestamp);
    }

    fn require_reward_not_distributed(&self, epoch: u64, token_identifier: &TokenIdentifier) {
        require!(
            self.reward_distribution_timestamp(epoch, token_identifier)
                .is_empty()
                && self.reward_rate(epoch, token_identifier).is_empty(),
            ERR_REWARD_ALREADY_DISTRIBUTED
        );
    }

    fn require_token_is_reward_token(&self, incoming_token_identifier: &TokenIdentifier) {
        require!(
            self.reward_token_identifiers()
                .contains(incoming_token_identifier),
            ERR_INVALID_REWARD_TOKEN_ID
        );
    }
}
