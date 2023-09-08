use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use crate::types::start_unbonding_payload::StartUnbondingPayload;

use super::{
    coding_division_sft_staking_module::CodingDivisionSftStakingModule,
    default::DefaultStakingModule, invalid::InvalidStakingModule,
};

multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, NestedDecode, NestedEncode)]
pub enum StakingModuleType {
    Invalid = 0,
    CodingDivisionSfts = 1,
    XBunnies = 2,
    Bloodshed = 3,
    Nosferatu = 4,
    VestaXDAO = 5,
}

pub enum StakingModuleTypeMapping<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    Invalid(InvalidStakingModule<'a, C>),
    CodingDivisionSfts(CodingDivisionSftStakingModule<'a, C>),
    XBunnies(DefaultStakingModule<'a, C>),
    Bloodshed(DefaultStakingModule<'a, C>),
    Nosferatu(DefaultStakingModule<'a, C>),
    VestaXDAO(DefaultStakingModule<'a, C>),
}

pub trait StakingModuleTypeFactory<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_module(
        &self,
        sc_ref: &'a C,
        token_identifier: TokenIdentifier<C::Api>,
        user_address: ManagedAddress<C::Api>,
    ) -> StakingModuleTypeMapping<'a, C>;
}

impl<'a, C> StakingModuleTypeFactory<'a, C> for StakingModuleType
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_module(
        &self,
        sc_ref: &'a C,
        token_identifier: TokenIdentifier<C::Api>,
        user_address: ManagedAddress<C::Api>,
    ) -> StakingModuleTypeMapping<'a, C> {
        match self {
            StakingModuleType::Invalid => {
                StakingModuleTypeMapping::Invalid(InvalidStakingModule::new())
            }
            StakingModuleType::CodingDivisionSfts => StakingModuleTypeMapping::CodingDivisionSfts(
                CodingDivisionSftStakingModule::new(sc_ref, token_identifier, user_address),
            ),
            StakingModuleType::XBunnies => StakingModuleTypeMapping::XBunnies(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address),
            ),
            StakingModuleType::Bloodshed => StakingModuleTypeMapping::Bloodshed(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address),
            ),
            StakingModuleType::Nosferatu => StakingModuleTypeMapping::Nosferatu(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address),
            ),
            StakingModuleType::VestaXDAO => StakingModuleTypeMapping::VestaXDAO(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address),
            ),
        }
    }
}

pub trait VestaStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self) -> BigUint<C::Api>;
    fn get_final_user_score(&self) -> BigUint<C::Api>;
    fn add_to_storage(&self, nonce: u64, amount: BigUint<C::Api>);
    fn start_unbonding(&self, payload: StartUnbondingPayload<C::Api>) -> bool;
    fn get_final_secondary_score(&self) -> BigUint<C::Api>;
}

impl<'a, C> VestaStakingModule<'a, C> for StakingModuleTypeMapping<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self) -> BigUint<C::Api> {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.get_base_user_score(),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => module.get_base_user_score(),
            StakingModuleTypeMapping::XBunnies(module) => module.get_base_user_score(),
            StakingModuleTypeMapping::Bloodshed(module) => module.get_base_user_score(),
            StakingModuleTypeMapping::Nosferatu(module) => module.get_base_user_score(),
            StakingModuleTypeMapping::VestaXDAO(module) => module.get_base_user_score(),
        }
    }

    fn get_final_user_score(&self) -> BigUint<<C>::Api> {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.get_final_user_score(),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => module.get_final_user_score(),
            StakingModuleTypeMapping::XBunnies(module) => module.get_final_user_score(),
            StakingModuleTypeMapping::Bloodshed(module) => module.get_final_user_score(),
            StakingModuleTypeMapping::Nosferatu(module) => module.get_final_user_score(),
            StakingModuleTypeMapping::VestaXDAO(module) => module.get_final_user_score(),
        }
    }

    fn add_to_storage(&self, nonce: u64, amount: BigUint<C::Api>) {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => {
                module.add_to_storage(nonce, amount)
            }
            StakingModuleTypeMapping::XBunnies(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::Bloodshed(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::Nosferatu(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::VestaXDAO(module) => module.add_to_storage(nonce, amount),
        }
    }

    fn start_unbonding(&self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::XBunnies(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::Bloodshed(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::Nosferatu(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::VestaXDAO(module) => module.start_unbonding(payload),
        }
    }

    fn get_final_secondary_score(&self) -> BigUint<<C>::Api> {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.get_final_secondary_score(),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => {
                module.get_final_secondary_score()
            }
            StakingModuleTypeMapping::XBunnies(module) => module.get_final_secondary_score(),
            StakingModuleTypeMapping::Bloodshed(module) => module.get_final_secondary_score(),
            StakingModuleTypeMapping::Nosferatu(module) => module.get_final_secondary_score(),
            StakingModuleTypeMapping::VestaXDAO(module) => module.get_final_secondary_score(),
        }
    }
}
