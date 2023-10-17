use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use super::{
    default::DefaultStakingModule,
    staking_module_type::{StakingModuleType, VestaStakingModule},
};
use crate::types::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload};

pub struct SnakesSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
    default_impl: DefaultStakingModule<'a, C>,
    user_address: ManagedAddress<C::Api>,
    initial_general_shares_score: BigUint<C::Api>,
    initial_user_shares_score: BigUint<C::Api>,
}

impl<'a, C> SnakesSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    pub fn new(
        sc_ref: &'a C,
        impl_token_id: TokenIdentifier<C::Api>,
        user_address: ManagedAddress<C::Api>,
        module_type: StakingModuleType,
    ) -> Self {
        let initial_general_shares_score = sc_ref
            .aggregated_staking_score(&StakingModuleType::SharesSfts)
            .get();
        let initial_user_shares_score = sc_ref
            .raw_aggregated_user_staking_score(&StakingModuleType::SharesSfts, &user_address)
            .get();
        let default_impl = DefaultStakingModule::new(
            sc_ref,
            impl_token_id.clone(),
            user_address.clone(),
            module_type,
        );

        Self {
            sc_ref,
            impl_token_id,
            default_impl,
            initial_general_shares_score,
            initial_user_shares_score,
            user_address,
        }
    }
}

impl<'a, C> VestaStakingModule<'a, C> for SnakesSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self, staking_module_type: &StakingModuleType) -> BigUint<C::Api> {
        // Snakes don't get any share from the primary reward consisting of 15% of total AURYN emission
        // because they get a share from the secondary reward consisting of 5% of total AURYN emission
        // Hence why the base score for "All" is 0
        if staking_module_type == &StakingModuleType::All {
            return BigUint::zero();
        }

        let mut snakes_score = BigUint::zero();
        let base_score = BigUint::from(
            self.sc_ref
                .base_asset_score(&self.impl_token_id, staking_module_type)
                .get(),
        );

        let mut shares_score = BigUint::<C::Api>::zero();

        for staked_nft_info in self.default_impl.staked_assets.iter() {
            if staked_nft_info.nonce == 1 {
                let asset_nonce_score = self.sc_ref.nonce_asset_score(
                    &self.impl_token_id,
                    staked_nft_info.nonce,
                    staking_module_type,
                );

                let unit_score;
                if !asset_nonce_score.is_empty() {
                    unit_score = BigUint::from(asset_nonce_score.get());
                } else {
                    unit_score = base_score.clone();
                }

                snakes_score += &unit_score * &staked_nft_info.quantity;
            }

            let nonce_shares_score = self
                .sc_ref
                .nonce_asset_score(
                    &self.impl_token_id,
                    staked_nft_info.nonce,
                    &StakingModuleType::SharesSfts,
                )
                .get();
            shares_score += &BigUint::from(nonce_shares_score) * &staked_nft_info.quantity;
        }

        self.update_shares_score(&shares_score);

        snakes_score
    }

    fn add_to_storage(&mut self, nonce: u64, amount: BigUint<C::Api>) {
        self.default_impl.add_to_storage(nonce, amount);
    }

    fn start_unbonding(&mut self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        self.default_impl.start_unbonding(payload)
    }
}

impl<'a, C> SnakesSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn update_shares_score(&self, new_user_score: &BigUint<C::Api>) {
        if &self.initial_user_shares_score == new_user_score {
            return;
        }
        let new_aggregated_score =
            &self.initial_general_shares_score - &self.initial_user_shares_score + new_user_score;

        self.sc_ref
            .aggregated_staking_score(&StakingModuleType::SharesSfts)
            .set(&new_aggregated_score);
        self.sc_ref
            .raw_aggregated_user_staking_score(&StakingModuleType::SharesSfts, &self.user_address)
            .set(new_user_score);
        self.sc_ref
            .aggregated_user_staking_score(&StakingModuleType::SharesSfts, &self.user_address)
            .set(new_user_score);
    }
}
