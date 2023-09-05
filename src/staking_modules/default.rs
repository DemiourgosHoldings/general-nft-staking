use multiversx_sc::types::{BigUint, ManagedAddress, ManagedVec, TokenIdentifier};

use crate::{
    constants::DEB_DENOMINATION,
    types::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload},
};

use super::staking_module_type::VestaStakingModule;

pub struct DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
    user_address: ManagedAddress<C::Api>,
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
    ) -> Self {
        Self {
            sc_ref,
            impl_token_id,
            user_address,
        }
    }
}

impl<'a, C> VestaStakingModule<'a, C> for DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self) -> BigUint<C::Api> {
        let staked_nft_nonces = self
            .sc_ref
            .staked_nfts(&self.impl_token_id)
            .get(&self.user_address)
            .unwrap_or_else(|| ManagedVec::new());

        let mut score = BigUint::zero();
        let base_score = BigUint::from(self.sc_ref.base_asset_score(&self.impl_token_id).get());

        for staked_nft_info in staked_nft_nonces.iter() {
            let asset_nonce_score = self
                .sc_ref
                .nonce_asset_score(&self.impl_token_id, staked_nft_info.nonce);

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

    fn get_final_user_score(&self) -> BigUint<C::Api> {
        let base_score = self.get_base_user_score();
        let deb_denomination = BigUint::from(DEB_DENOMINATION);

        let user_deb = match self.sc_ref.user_deb(&self.user_address).is_empty() {
            false => self.sc_ref.user_deb(&self.user_address).get(),
            true => deb_denomination.clone(),
        };

        &base_score * &user_deb / deb_denomination
    }

    fn add_to_storage(&self, nonce: u64, amount: BigUint<C::Api>) {
        let mut staked_nfts = self
            .sc_ref
            .staked_nfts(&self.impl_token_id)
            .remove(&self.user_address)
            .unwrap_or_else(|| ManagedVec::new());

        staked_nfts.push(NonceQtyPair {
            nonce: nonce,
            quantity: amount,
        });

        self.sc_ref
            .staked_nfts(&self.impl_token_id)
            .insert(self.user_address.clone(), staked_nfts);
    }

    fn start_unbonding(&self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        let staked_nfts = self
            .sc_ref
            .staked_nfts(&payload.token_identifier)
            .remove(&self.user_address)
            .unwrap_or_else(|| ManagedVec::new());
        let initial_staked_nfts_count = staked_nfts.len();

        let mut remaining_staked_nfts = ManagedVec::new();
        for staked_nft in staked_nfts.iter() {
            let unstake_nonce_quantity = payload.get_nonce_quantity(staked_nft.nonce);
            if &unstake_nonce_quantity == &BigUint::zero() {
                remaining_staked_nfts.push(staked_nft);
                continue;
            }

            if &unstake_nonce_quantity > &staked_nft.quantity {
                return false;
            }

            if &staked_nft.quantity == &unstake_nonce_quantity {
                continue;
            }

            remaining_staked_nfts.push(NonceQtyPair {
                nonce: staked_nft.nonce,
                quantity: &staked_nft.quantity - &unstake_nonce_quantity,
            });
        }

        let remaining_staked_nfts_count = remaining_staked_nfts.len();
        self.sc_ref
            .staked_nfts(&payload.token_identifier)
            .insert(self.user_address.clone(), remaining_staked_nfts);

        initial_staked_nfts_count != remaining_staked_nfts_count
    }
}
