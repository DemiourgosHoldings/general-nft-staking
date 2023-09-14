use multiversx_sc::types::{BigUint, ManagedAddress, ManagedVec, TokenIdentifier};

use crate::types::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload};

use super::staking_module_type::{StakingModuleType, VestaStakingModule};

pub struct DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
    user_address: ManagedAddress<C::Api>,
    pub module_type: StakingModuleType,
    pub staked_assets: ManagedVec<C::Api, NonceQtyPair<C::Api>>,
}

impl<'a, C> DefaultStakingModule<'a, C>
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
        let staked_assets = Self::get_staked_assets(sc_ref, &impl_token_id, &user_address);
        Self {
            sc_ref,
            impl_token_id,
            user_address,
            module_type,
            staked_assets,
        }
    }

    //TODO: add a struct field and store this data there instead of reading from storage everytime
    //TODO: add a default DROP fn that stores the changes in this struct field to storage
    pub fn get_staked_nfts_data(&self) -> ManagedVec<C::Api, NonceQtyPair<C::Api>> {
        Self::get_staked_assets(&self.sc_ref, &self.impl_token_id, &self.user_address)
    }

    fn get_staked_assets(
        sc_ref: &'a C,
        impl_token_id: &TokenIdentifier<C::Api>,
        user_address: &ManagedAddress<C::Api>,
    ) -> ManagedVec<C::Api, NonceQtyPair<C::Api>> {
        sc_ref
            .staked_nfts(&impl_token_id)
            .remove(&user_address)
            .unwrap_or_else(|| ManagedVec::new())
    }
}

impl<'a, C> VestaStakingModule<'a, C> for DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self, staking_module_type: &StakingModuleType) -> BigUint<C::Api> {
        let mut score = BigUint::zero();
        let base_score = BigUint::from(
            self.sc_ref
                .base_asset_score(&self.impl_token_id, staking_module_type)
                .get(),
        );
        for staked_nft_info in self.staked_assets.iter() {
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

            score += &unit_score * &staked_nft_info.quantity;
        }

        score
    }

    fn add_to_storage(&mut self, nonce: u64, quantity: BigUint<C::Api>) {
        self.staked_assets.push(NonceQtyPair { nonce, quantity });
    }

    fn start_unbonding(&mut self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        let mut total_unstaked_quantity = BigUint::zero();

        let mut remaining_staked_nfts = ManagedVec::new();
        for staked_nft in self.staked_assets.iter() {
            let unstake_nonce_quantity = payload.get_nonce_quantity(staked_nft.nonce);
            if &unstake_nonce_quantity == &BigUint::zero() {
                remaining_staked_nfts.push(staked_nft);
                continue;
            }

            if &unstake_nonce_quantity > &staked_nft.quantity {
                return false;
            }

            if &staked_nft.quantity == &unstake_nonce_quantity {
                total_unstaked_quantity += &staked_nft.quantity;
                continue;
            }

            total_unstaked_quantity += &unstake_nonce_quantity;
            remaining_staked_nfts.push(NonceQtyPair {
                nonce: staked_nft.nonce,
                quantity: &staked_nft.quantity - &unstake_nonce_quantity,
            });
        }

        self.staked_assets = remaining_staked_nfts;

        &total_unstaked_quantity > &0
    }
}

impl<'a, C> Drop for DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn drop(&mut self) {
        // commit changes to storage for the mutable fields
        self.sc_ref
            .staked_nfts(&self.impl_token_id)
            .insert(self.user_address.clone(), self.staked_assets.clone());
    }
}
