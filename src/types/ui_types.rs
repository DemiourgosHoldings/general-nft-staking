use super::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload};

multiversx_sc::derive_imports!();
multiversx_sc::imports!();

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode,
)]
pub struct UIAggregatedPoolScore<M: ManagedTypeApi> {
    pub pool_type: u8,
    pub pool_score: BigUint<M>,
}

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode,
)]
pub struct UIExtendedAggregatedPoolScore<M: ManagedTypeApi> {
    pub pool_type: u8,
    pub pool_score: BigUint<M>,
    pub raw_pool_score: BigUint<M>,
}

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode,
)]
pub struct UIUserPoolData<M: ManagedTypeApi> {
    pub pool_token_identifier: TokenIdentifier<M>,
    pub pool_score_data: UIExtendedAggregatedPoolScore<M>,
    pub pool_staked_assets: ManagedVec<M, NonceQtyPair<M>>,
}

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode,
)]
pub struct UIUnbondingAsset<M: ManagedTypeApi> {
    pub timestamp: u64,
    pub assets: StartUnbondingPayload<M>,
}

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode,
)]
pub struct UIUserDataPayload<M: ManagedTypeApi> {
    pub pending_rewards: ManagedVec<M, EsdtTokenPayment<M>>,
    pub user_pool_data: ManagedVec<M, UIUserPoolData<M>>,
    pub unbonding_assets: ManagedVec<M, UIUnbondingAsset<M>>,
    pub user_deb: BigUint<M>,
}
