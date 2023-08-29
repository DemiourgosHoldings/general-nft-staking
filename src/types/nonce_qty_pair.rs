multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode,
)]
pub struct NonceQtyPair<M: ManagedTypeApi> {
    pub nonce: u64,
    pub quantity: BigUint<M>,
}
