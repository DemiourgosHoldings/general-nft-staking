use multiversx_sc::types::{BigUint, ManagedAddress, ManagedVec, TokenIdentifier};

use crate::{constants::DEB_DENOMINATION, types::nonce_qty_pair::NonceQtyPair};

use super::staking_module_type::VestaStakingModule;

pub struct DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
    user_address: ManagedAddress<C::Api>,
}

impl<'a, C> DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
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
}
