use super::nonce_qty_pair::NonceQtyPair;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, Clone, PartialEq, Eq, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode,
)]
pub struct StartUnbondingPayload<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub items: ManagedVec<M, NonceQtyPair<M>>,
}

impl<M: ManagedTypeApi> StartUnbondingPayload<M> {
    pub fn new(
        token_identifier: TokenIdentifier<M>,
        items: ManagedVec<M, NonceQtyPair<M>>,
    ) -> Self {
        Self {
            token_identifier,
            items,
        }
    }

    pub fn get_nonce_quantity(&self, nonce: u64) -> BigUint<M> {
        let item = self.items.iter().find(|p| p.nonce == nonce);
        if item.is_none() {
            return BigUint::zero();
        }
        item.unwrap().quantity
    }
}
