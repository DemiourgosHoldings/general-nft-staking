use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use super::{
    default::DefaultStakingModule,
    staking_module_type::{StakingModuleType, VestaStakingModule},
};
use crate::{
    constants::VESTA_CODING_DIVISION_FULL_SET_MAX_NONCE,
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
            user_address,
            module_type,
        );
        Self {
            sc_ref,
            impl_token_id,
            default_impl,
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

    fn add_to_storage(&mut self, nonce: u64, amount: BigUint<C::Api>) {
        let existing_item_index = self
            .default_impl
            .staked_assets
            .iter()
            .position(|p| p.nonce == nonce);
        let item_to_insert;
        if existing_item_index.is_none() {
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: amount,
            };
        } else {
            let index_to_remove = existing_item_index.unwrap();
            let existing_item = self.default_impl.staked_assets.get(index_to_remove);
            self.default_impl.staked_assets.remove(index_to_remove);
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: existing_item.quantity + amount,
            };
        }

        self.default_impl.staked_assets.push(item_to_insert);
    }

    fn start_unbonding(&mut self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        self.default_impl.start_unbonding(payload)
    }
}
