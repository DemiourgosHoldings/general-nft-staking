use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use crate::types::start_unbonding_payload::StartUnbondingPayload;

use super::{
    coding_division_sft_staking_module::CodingDivisionSftStakingModule,
    default::DefaultStakingModule, invalid::InvalidStakingModule,
    snakes_sft_staking_module::SnakesSftStakingModule,
};

multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, NestedDecode, NestedEncode, Debug,
)]
pub enum StakingModuleType {
    Invalid = 0,
    CodingDivisionSfts = 1,
    XBunnies = 2,
    Bloodshed = 3,
    Nosferatu = 4,
    VestaXDAO = 5,
    SnakesSfts = 6,
    SharesSfts = 7,

    All = 100,
}

impl StakingModuleType {
    pub fn iter() -> &'static [Self] {
        &[
            Self::Invalid,
            Self::CodingDivisionSfts,
            Self::XBunnies,
            Self::Bloodshed,
            Self::Nosferatu,
            Self::VestaXDAO,
            Self::SnakesSfts,
            Self::SharesSfts,
            Self::All,
        ]
    }
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
    SnakesSfts(SnakesSftStakingModule<'a, C>),
    SharesSfts(DefaultStakingModule<'a, C>),

    All(DefaultStakingModule<'a, C>),
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
            StakingModuleType::CodingDivisionSfts => {
                StakingModuleTypeMapping::CodingDivisionSfts(CodingDivisionSftStakingModule::new(
                    sc_ref,
                    token_identifier,
                    user_address,
                    self.clone(),
                ))
            }
            StakingModuleType::XBunnies => StakingModuleTypeMapping::XBunnies(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address, self.clone()),
            ),
            StakingModuleType::Bloodshed => StakingModuleTypeMapping::Bloodshed(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address, self.clone()),
            ),
            StakingModuleType::Nosferatu => StakingModuleTypeMapping::Nosferatu(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address, self.clone()),
            ),
            StakingModuleType::VestaXDAO => StakingModuleTypeMapping::VestaXDAO(
                DefaultStakingModule::new(sc_ref, token_identifier, user_address, self.clone()),
            ),
            StakingModuleType::SnakesSfts => StakingModuleTypeMapping::SnakesSfts(
                SnakesSftStakingModule::new(sc_ref, token_identifier, user_address, self.clone()),
            ),
            StakingModuleType::SharesSfts => StakingModuleTypeMapping::SnakesSfts(
                SnakesSftStakingModule::new(sc_ref, token_identifier, user_address, self.clone()),
            ),
            StakingModuleType::All => StakingModuleTypeMapping::All(DefaultStakingModule::new(
                sc_ref,
                token_identifier,
                user_address,
                self.clone(),
            )),
        }
    }
}

pub trait VestaStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self, module_type: &StakingModuleType) -> BigUint<C::Api>;
    fn add_to_storage(&mut self, nonce: u64, amount: BigUint<C::Api>);
    fn start_unbonding(&mut self, payload: StartUnbondingPayload<C::Api>) -> bool;
}

impl<'a, C> VestaStakingModule<'a, C> for StakingModuleTypeMapping<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self, module_type: &StakingModuleType) -> BigUint<C::Api> {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.get_base_user_score(module_type),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => {
                module.get_base_user_score(module_type)
            }
            StakingModuleTypeMapping::XBunnies(module) => module.get_base_user_score(module_type),
            StakingModuleTypeMapping::Bloodshed(module) => module.get_base_user_score(module_type),
            StakingModuleTypeMapping::Nosferatu(module) => module.get_base_user_score(module_type),
            StakingModuleTypeMapping::VestaXDAO(module) => module.get_base_user_score(module_type),
            StakingModuleTypeMapping::SnakesSfts(module) => module.get_base_user_score(module_type),
            StakingModuleTypeMapping::SharesSfts(module) => module.get_base_user_score(module_type),
            StakingModuleTypeMapping::All(module) => module.get_base_user_score(module_type),
        }
    }

    fn add_to_storage(&mut self, nonce: u64, amount: BigUint<C::Api>) {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => {
                module.add_to_storage(nonce, amount)
            }
            StakingModuleTypeMapping::XBunnies(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::Bloodshed(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::Nosferatu(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::VestaXDAO(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::SnakesSfts(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::SharesSfts(module) => module.add_to_storage(nonce, amount),
            StakingModuleTypeMapping::All(module) => module.add_to_storage(nonce, amount),
        }
    }

    fn start_unbonding(&mut self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        match self {
            StakingModuleTypeMapping::Invalid(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::CodingDivisionSfts(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::XBunnies(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::Bloodshed(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::Nosferatu(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::VestaXDAO(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::SnakesSfts(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::SharesSfts(module) => module.start_unbonding(payload),
            StakingModuleTypeMapping::All(module) => module.start_unbonding(payload),
        }
    }
}
