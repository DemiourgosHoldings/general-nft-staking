use multiversx_sc::types::{BigUint, ManagedAddress, ManagedVec, TokenIdentifier};

use crate::constants::DEB_DENOMINATION;

use super::staking_module_type::VestaStakingModule;

pub struct DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
}

impl<'a, C> DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    pub fn new(sc_ref: &'a C, impl_token_id: TokenIdentifier<C::Api>) -> Self {
        Self {
            sc_ref,
            impl_token_id,
        }
    }
}

impl<'a, C> VestaStakingModule<'a, C> for DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    fn get_base_user_score(&self, address: &ManagedAddress<C::Api>) -> BigUint<C::Api> {
        let staked_nft_nonces = self
            .sc_ref
            .staked_nfts(&self.impl_token_id)
            .get(address)
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

    fn get_final_user_score(&self, address: &ManagedAddress<C::Api>) -> BigUint<C::Api> {
        let base_score = self.get_base_user_score(address);
        let deb_denomination = BigUint::from(DEB_DENOMINATION);

        let user_deb = match self.sc_ref.user_deb(address).is_empty() {
            false => self.sc_ref.user_deb(address).get(),
            true => deb_denomination.clone(),
        };

        &base_score * &user_deb / deb_denomination
    }
}
