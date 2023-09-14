use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use super::{
    default::DefaultStakingModule,
    staking_module_type::{StakingModuleType, VestaStakingModule},
};
use crate::{
    constants::{DEB_DENOMINATION, VESTA_CODING_DIVISION_FULL_SET_MAX_NONCE},
    types::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload},
};

pub struct CodingDivisionSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
    default_impl: DefaultStakingModule<'a, C>,
    user_address: ManagedAddress<C::Api>,
}

impl<'a, C> CodingDivisionSftStakingModule<'a, C>
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
            user_address,
        }
    }

    fn count_full_sets(&self) -> BigUint<C::Api> {
        let staked_nft_nonces = self.default_impl.get_staked_nfts_data();

        let mut full_sets = BigUint::from(100_000u32);

        for set_nonce in 1..=VESTA_CODING_DIVISION_FULL_SET_MAX_NONCE {
            let item = staked_nft_nonces.iter().find(|p| p.nonce == set_nonce);
            if item.is_none() {
                return BigUint::zero();
            }
            let item_quantity = item.unwrap().quantity;
            if item_quantity < full_sets {
                full_sets = item_quantity;
            }
        }

        full_sets
    }

    fn apply_full_set_bonus(&self, default_base_score: BigUint<C::Api>) -> BigUint<C::Api> {
        let full_sets = self.count_full_sets();
        if &full_sets == &0 {
            return default_base_score;
        }

        let full_set_score = match self
            .sc_ref
            .full_set_score(&self.impl_token_id, &StakingModuleType::All)
            .is_empty()
        {
            true => BigUint::zero(),
            false => BigUint::from(
                self.sc_ref
                    .full_set_score(&self.impl_token_id, &StakingModuleType::All)
                    .get(),
            ),
        };

        default_base_score + &full_sets * &full_set_score
    }
}

impl<'a, C> VestaStakingModule<'a, C> for CodingDivisionSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self, staking_module_type: &StakingModuleType) -> BigUint<C::Api> {
        let default_base_score = self.default_impl.get_base_user_score(&staking_module_type);

        self.apply_full_set_bonus(default_base_score)
    }

    fn get_final_user_score(&self) -> BigUint<C::Api> {
        let base_score = self.get_base_user_score(&self.default_impl.module_type);
        let deb_denomination = BigUint::from(DEB_DENOMINATION);

        let user_deb = match self.sc_ref.user_deb(&self.user_address).is_empty() {
            false => self.sc_ref.user_deb(&self.user_address).get(),
            true => deb_denomination.clone(),
        };

        &base_score * &user_deb / deb_denomination
    }

    fn add_to_storage(&self, nonce: u64, amount: BigUint<C::Api>) {
        let mut staked_nfts = self.default_impl.get_staked_nfts_data();

        let existing_item_index = staked_nfts.iter().position(|p| p.nonce == nonce);
        let item_to_insert;
        if existing_item_index.is_none() {
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: amount,
            };
        } else {
            let index_to_remove = existing_item_index.unwrap();
            let existing_item = staked_nfts.get(index_to_remove);
            staked_nfts.remove(index_to_remove);
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: existing_item.quantity + amount,
            };
        }

        staked_nfts.push(item_to_insert);

        self.sc_ref
            .staked_nfts(&self.impl_token_id)
            .insert(self.user_address.clone(), staked_nfts);
    }

    fn start_unbonding(&self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        self.default_impl.start_unbonding(payload)
    }

    fn get_final_secondary_score(&self) -> BigUint<<C>::Api> {
        self.default_impl
            .get_user_score_temp(&StakingModuleType::CodingDivisionSfts)
    }
}
