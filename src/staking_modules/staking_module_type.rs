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
