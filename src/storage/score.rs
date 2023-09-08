multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ScoreStorageModule {
    #[view(getBaseAssetScore)]
    #[storage_mapper("base_asset_score")]
    fn base_asset_score(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<usize>;

    #[view(getNonceAssetScore)]
    #[storage_mapper("nonce_asset_score")]
    fn nonce_asset_score(
        &self,
        token_identifier: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<usize>;

    #[view(getFullSetScore)]
    #[storage_mapper("full_set_score")]
    fn full_set_score(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<usize>;

    #[view(getSecondaryBaseAssetScore)]
    #[storage_mapper("secondary_base_asset_score")]
    fn secondary_base_asset_score(
        &self,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<usize>;

    #[view(getSecondaryNonceAssetScore)]
    #[storage_mapper("secondary_nonce_asset_score")]
    fn secondary_nonce_asset_score(
        &self,
        token_identifier: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<usize>;
}
