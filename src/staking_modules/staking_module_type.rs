use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use super::default::DefaultStakingModule;

multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi)]
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
{
    Invalid,
    CodingDivisionSfts(DefaultStakingModule<'a, C>),
    XBunnies(DefaultStakingModule<'a, C>),
    Bloodshed(DefaultStakingModule<'a, C>),
    Nosferatu(DefaultStakingModule<'a, C>),
    VestaXDAO(DefaultStakingModule<'a, C>),
}

pub trait StakingModuleTypeFactory<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    fn get_module(
        &self,
        sc_ref: &'a C,
        token_identifier: TokenIdentifier<C::Api>,
    ) -> StakingModuleTypeMapping<'a, C>;
}

impl<'a, C> StakingModuleTypeFactory<'a, C> for StakingModuleType
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    fn get_module(
        &self,
        sc_ref: &'a C,
        token_identifier: TokenIdentifier<C::Api>,
    ) -> StakingModuleTypeMapping<'a, C> {
        match self {
            StakingModuleType::Invalid => StakingModuleTypeMapping::Invalid,
            StakingModuleType::CodingDivisionSfts => StakingModuleTypeMapping::CodingDivisionSfts(
                DefaultStakingModule::new(sc_ref, token_identifier),
            ),
            StakingModuleType::XBunnies => StakingModuleTypeMapping::XBunnies(
                DefaultStakingModule::new(sc_ref, token_identifier),
            ),
            StakingModuleType::Bloodshed => StakingModuleTypeMapping::Bloodshed(
                DefaultStakingModule::new(sc_ref, token_identifier),
            ),
            StakingModuleType::Nosferatu => StakingModuleTypeMapping::Nosferatu(
                DefaultStakingModule::new(sc_ref, token_identifier),
            ),
            StakingModuleType::VestaXDAO => StakingModuleTypeMapping::VestaXDAO(
                DefaultStakingModule::new(sc_ref, token_identifier),
            ),
        }
    }
}

pub trait VestaStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    fn get_base_user_score(&self, address: &ManagedAddress<C::Api>) -> BigUint<C::Api>;
    fn get_final_user_score(&self, address: &ManagedAddress<C::Api>) -> BigUint<C::Api>;
}
