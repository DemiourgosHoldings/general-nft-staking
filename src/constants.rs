pub const DEB_DENOMINATION: u64 = 100_000;
pub const VESTA_CODING_DIVISION_FULL_SET_MAX_NONCE: u64 = 10;

pub const DEFAULT_UNBONDING_TIME_PENALTY: u64 = 3 * 24 * 3600; // three days

// error messages
pub const ERR_FAILED_UNBONDING: &str = "Unbonding failed";
pub const ERR_ONE_TOKEN_ID_SUPPORTED: &str = "Only one token id is allowed per TX";
pub const ERR_NOTHING_TO_CLAIM: &str = "Nothing to claim";
pub const ERR_REWARD_ALREADY_DISTRIBUTED: &str = "Reward already distributed";
pub const ERR_INVALID_REWARD_TOKEN_ID: &str = "Invalid reward token id";
pub const ERR_COLLECTION_ALREADY_REGISTERED: &str = "Collection already registered";
pub const ERR_INVALID_STAKED_TOKEN_ID: &str = "Invalid token identifier";
