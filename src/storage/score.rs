use crate::staking_modules::staking_module_type::StakingModuleType;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ScoreStorageModule {
    #[view(getBaseAssetScore)]
    #[storage_mapper("base_asset_score")]
    fn base_asset_score(
        &self,
        collection_token_identifier: &TokenIdentifier,
        staking_module: &StakingModuleType,
    ) -> SingleValueMapper<usize>;

    #[view(getNonceAssetScore)]
    #[storage_mapper("nonce_asset_score")]
    fn nonce_asset_score(
        &self,
        collection_token_identifier: &TokenIdentifier,
        nonce: u64,
        staking_module: &StakingModuleType,
    ) -> SingleValueMapper<usize>;

    #[view(getFullSetScore)]
    #[storage_mapper("full_set_score")]
    fn full_set_score(
        &self,
        collection_token_identifier: &TokenIdentifier,
        staking_module: &StakingModuleType,
    ) -> SingleValueMapper<usize>;
}
