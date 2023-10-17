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
    pub impl_token_id: TokenIdentifier<C::Api>,
    pub user_address: ManagedAddress<C::Api>,
    pub module_type: StakingModuleType,
    // pub staked_assets: ManagedVec<C::Api, NonceQtyPair<C::Api>>,
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
        // let staked_assets = Self::get_staked_assets(sc_ref, &impl_token_id, &user_address);
        Self {
            sc_ref,
            impl_token_id,
            user_address,
            module_type,
            // staked_assets,
        }
    }

    fn get_staked_assets(
        sc_ref: &'a C,
        impl_token_id: &TokenIdentifier<C::Api>,
        user_address: &ManagedAddress<C::Api>,
    ) -> ManagedVec<C::Api, NonceQtyPair<C::Api>> {
        sc_ref.get_staked_nfts(user_address, impl_token_id)
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
        let assets = Self::get_staked_assets(&self.sc_ref, &self.impl_token_id, &self.user_address);
        for staked_nft_info in assets.iter() {
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
        self.sc_ref
            .staked_nfts(&self.user_address, &self.impl_token_id)
            .entry(nonce)
            .and_modify(|old_qty| *old_qty += &quantity)
            .or_insert(quantity);
    }

    fn start_unbonding(&mut self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        let mut total_unstaked_quantity = BigUint::zero();
        let mut staked_assets_storage = self
            .sc_ref
            .staked_nfts(&self.user_address, &self.impl_token_id);
        for item in payload.items.iter() {
            let matching_staked_nft = staked_assets_storage.remove(&item.nonce);
            if matching_staked_nft.is_none() {
                // unbonding should fail
                return false;
            }

            let matching_staked_nft = matching_staked_nft.unwrap();
            if &matching_staked_nft < &item.quantity {
                // unbonding should fail
                return false;
            }

            total_unstaked_quantity += &item.quantity;
            if &matching_staked_nft == &item.quantity {
                // don't add this back to storage
                continue;
            }

            staked_assets_storage.insert(item.nonce, &matching_staked_nft - &item.quantity);
        }

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
        // self.sc_ref
        //     .staked_nfts(&self.impl_token_id)
        //     .insert(self.user_address.clone(), self.staked_assets.clone());
    }
}
