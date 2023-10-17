use crate::{
    constants::{
        DEB_DENOMINATION, ERR_CANNOT_REGISTER_AS_ALL, ERR_COLLECTION_ALREADY_REGISTERED,
        ERR_INVALID_REWARD_TOKEN_ID, ERR_REWARD_ALREADY_DISTRIBUTED,
    },
    staking_modules::staking_module_type::StakingModuleType,
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
        let total_score = self.aggregated_staking_score(&StakingModuleType::All).get();
        let payment = self.call_value().single_esdt();

        self.distribute_reward_handler(&StakingModuleType::All, payment, total_score);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(distributeSecondaryReward)]
    fn distribute_secondary_reward(&self, target: TokenIdentifier) {
        let staking_module_type = self.stake_pool_type_configuration(&target).get();
        let total_score = self.aggregated_staking_score(&staking_module_type).get();
        let payment = self.call_value().single_esdt();

        self.distribute_reward_handler(&staking_module_type, payment, total_score);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(distributeCompanyShareReward)]
    fn distribute_company_share_reward(&self) {
        let staking_module_type = StakingModuleType::SharesSfts;
        let total_score = self.aggregated_staking_score(&staking_module_type).get();
        let payment = self.call_value().single_esdt();

        self.distribute_reward_handler(&staking_module_type, payment, total_score);
    }

    #[only_owner]
    #[endpoint(updateDeb)]
    fn update_deb(&self, user_address: ManagedAddress, new_deb_val: BigUint) {
        let deb_denomination = BigUint::from(DEB_DENOMINATION);
        let mut old_deb = self.user_deb(&user_address).get();
        if &old_deb < &deb_denomination {
            old_deb = deb_denomination.clone();
        }
        let new_deb = match &new_deb_val < &deb_denomination {
            true => deb_denomination.clone(),
            false => new_deb_val.clone(),
        };

        if &old_deb == &new_deb {
            return;
        }

        for reward_token_id in self.reward_token_identifiers().iter() {
            let staking_module_type = self.stake_pool_type_configuration(&reward_token_id).get();
            secure_rewards(self, &user_address, &reward_token_id, &staking_module_type);

            self.update_score_handler(
                &staking_module_type,
                &user_address,
                &new_deb,
                &deb_denomination,
            );
        }

        secure_rewards(
            self,
            &user_address,
            &self.primary_reward_token_identifier().get(),
            &StakingModuleType::All,
        );

        self.update_score_handler(
            &StakingModuleType::All,
            &user_address,
            &new_deb,
            &deb_denomination,
        );
    }

    fn update_score_handler(
        &self,
        staking_module_type: &StakingModuleType,
        user_address: &ManagedAddress,
        new_deb: &BigUint,
        deb_denomination: &BigUint,
    ) {
        let current_score = self
            .raw_aggregated_user_staking_score(staking_module_type, &user_address)
            .get();

        if &current_score == &0 {
            return;
        }

        let current_score_with_deb = self
            .aggregated_user_staking_score(staking_module_type, &user_address)
            .get();
        let current_general_score = self.aggregated_staking_score(staking_module_type).get();
        let new_score_after_deb = &current_score * new_deb / deb_denomination;

        self.aggregated_user_staking_score(staking_module_type, &user_address)
            .set(&new_score_after_deb);
        self.aggregated_staking_score(staking_module_type)
            .set(&current_general_score - &current_score_with_deb + &new_score_after_deb);
    }

    fn distribute_reward_handler(
        &self,
        staking_module_type: &StakingModuleType,
        payment: EsdtTokenPayment,
        total_score: BigUint,
    ) {
        self.require_token_is_reward_token(&payment.token_identifier);
        let block_epoch = self.blockchain().get_block_epoch();
        let block_timestamp = self.blockchain().get_block_timestamp();

        self.require_reward_not_distributed(
            block_epoch,
            staking_module_type,
            &payment.token_identifier,
        );
        let reward_rate = payment.amount / total_score;

        self.reward_rate(block_epoch, staking_module_type, &payment.token_identifier)
            .set(reward_rate);
        self.reward_distribution_timestamp(block_epoch, &payment.token_identifier)
            .set(&block_timestamp);
    }

    fn require_reward_not_distributed(
        &self,
        epoch: u64,
        staking_module_type: &StakingModuleType,
        token_identifier: &TokenIdentifier,
    ) {
        require!(
            self.reward_distribution_timestamp(epoch, token_identifier)
                .is_empty()
                && self
                    .reward_rate(epoch, staking_module_type, token_identifier)
                    .is_empty(),
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

    #[only_owner]
    #[endpoint(createPool)]
    fn register_new_staking_pool(
        &self,
        collection_token_identifier: TokenIdentifier,
        staking_module_type: StakingModuleType,
    ) {
        require!(
            &staking_module_type != &StakingModuleType::All,
            ERR_CANNOT_REGISTER_AS_ALL
        );
        require!(
            self.stake_pool_type_configuration(&collection_token_identifier)
                .is_empty(),
            ERR_COLLECTION_ALREADY_REGISTERED
        );
        self.stake_pool_type_configuration(&collection_token_identifier)
            .set(staking_module_type);
        require!(
            self.eligible_stake_token_identifiers()
                .insert(collection_token_identifier),
            ERR_COLLECTION_ALREADY_REGISTERED
        );
    }

    #[only_owner]
    #[endpoint(overridePoolType)]
    fn override_stake_pool_type(
        &self,
        collection_token_identifier: TokenIdentifier,
        staking_module_type: StakingModuleType,
    ) {
        self.stake_pool_type_configuration(&collection_token_identifier)
            .set(staking_module_type);
    }

    #[only_owner]
    #[endpoint(setBaseAssetScore)]
    fn set_base_asset_score(
        &self,
        collection_token_identifier: &TokenIdentifier,
        staking_module: &StakingModuleType,
        score: usize,
    ) {
        self.base_asset_score(collection_token_identifier, staking_module)
            .set(&score);
    }

    #[only_owner]
    #[endpoint(setNonceAssetScore)]
    fn set_nonce_asset_score(
        &self,
        collection_token_identifier: &TokenIdentifier,
        staking_module: &StakingModuleType,
        score: usize,
        nonces: MultiValueEncoded<u64>,
    ) {
        for nonce in nonces.to_vec().iter() {
            self.nonce_asset_score(collection_token_identifier, nonce, staking_module)
                .set(&score);
        }
    }

    #[only_owner]
    #[endpoint(setNonceAssetScoreByRange)]
    fn set_nonce_asset_score_by_range(
        &self,
        collection_token_identifier: &TokenIdentifier,
        staking_module: &StakingModuleType,
        score: usize,
        nonce_range_start: u64,
        nonce_range_end: u64,
    ) {
        for nonce in nonce_range_start..=nonce_range_end {
            self.nonce_asset_score(collection_token_identifier, nonce, staking_module)
                .set(&score);
        }
    }

    #[only_owner]
    #[endpoint(registerRewardToken)]
    fn register_reward_token(
        &self,
        reward_token_identifier: TokenIdentifier,
        staking_module_type: StakingModuleType,
    ) {
        self.reward_token_to_staking_module_map(&reward_token_identifier)
            .insert(staking_module_type);
    }

    #[only_owner]
    #[endpoint(setFullSetScore)]
    fn set_full_set_score(
        &self,
        collection_token_identifier: &TokenIdentifier,
        staking_module: &StakingModuleType,
        score: usize,
    ) {
        self.full_set_score(collection_token_identifier, staking_module)
            .set(&score);
    }

    #[only_owner]
    #[endpoint(reset)]
    fn reset(
        &self,
        user: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
    ) {
        self.staked_nfts(&token_identifier).remove(&user);
        self.unbonding_assets(&user).clear();
        if &amount == &0 {
            return;
        }
        self.send()
            .direct_esdt(&user, &token_identifier, nonce, &amount);
    }
}
