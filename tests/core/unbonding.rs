use nft_staking::{
    constants::{DEFAULT_UNBONDING_TIME_PENALTY, ERR_FAILED_UNBONDING, ERR_NOTHING_TO_CLAIM},
    staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID, POOL2_QUANTITY_PER_NONCE, POOL2_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn single_successful_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1)], NO_ERR_MSG);
}

#[test]
fn partial_stake_successful_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1), (2, 1)], NO_ERR_MSG);
}

#[test]
fn full_stake_successful_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1), (2, 1), (3, 1)], NO_ERR_MSG);
}

#[test]
fn wrong_token_identifier_failed_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL2_TOKEN_ID, &[(1, 1)], ERR_FAILED_UNBONDING);
}

#[test]
fn correct_token_identifier_invalid_nonce_failed_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(4, 1)], ERR_FAILED_UNBONDING);
}

#[test]
#[allow(deprecated)]
fn unbonded_batch_successful_claim() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1)], NO_ERR_MSG);
    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY + 1);
    setup.claim_unbonded(NO_ERR_MSG);
    setup.assert_user_nft_balance(POOL1_TOKEN_ID, 1, 1);
}

#[test]
#[allow(deprecated)]
fn multiple_unbonded_batches_successful_claim() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);
    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1)], NO_ERR_MSG);
    setup.b_mock.set_block_timestamp(1);
    setup.start_unbonding(POOL1_TOKEN_ID, &[(2, 1)], NO_ERR_MSG);

    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY + 2);

    setup.claim_unbonded(NO_ERR_MSG);

    setup.assert_user_nft_balance(POOL1_TOKEN_ID, 1, 1);
    setup.assert_user_nft_balance(POOL1_TOKEN_ID, 2, 1);
}

#[test]
#[allow(deprecated)]
fn pending_batch_and_unbonded_batch_partial_successful_claim() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1)], NO_ERR_MSG);
    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY / 2);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(2, 1)], NO_ERR_MSG);
    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY + 1);

    setup.claim_unbonded(NO_ERR_MSG);
    setup.assert_user_nft_balance(POOL1_TOKEN_ID, 1, 1);
    setup.assert_user_nft_balance(POOL1_TOKEN_ID, 2, 0);
}

#[test]
#[allow(deprecated)]
fn no_batch_to_claim() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.claim_unbonded(ERR_NOTHING_TO_CLAIM);

    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY + 1);

    setup.claim_unbonded(ERR_NOTHING_TO_CLAIM);
}

#[test]
#[allow(deprecated)]
fn nothing_unbonded() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.claim_unbonded(ERR_NOTHING_TO_CLAIM);

    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY + 1);

    setup.claim_unbonded(ERR_NOTHING_TO_CLAIM);
}

#[test]
#[allow(deprecated)]
fn unbond_one_sft_nonce_includes_quantity() {
    let amount_to_stake = 5;
    let amount_to_unstake = 1;
    let score_per_unit = 5;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, amount_to_stake)];
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::CodingDivisionSfts);
    setup.set_token_score(
        StakingModuleType::CodingDivisionSfts,
        POOL2_TOKEN_ID,
        score_per_unit,
    );
    setup.stake(&transfers, NO_ERR_MSG);

    setup.assert_user_score(
        StakingModuleType::CodingDivisionSfts,
        amount_to_stake * score_per_unit as u64,
    );

    setup.start_unbonding(POOL2_TOKEN_ID, &[(1, amount_to_unstake)], NO_ERR_MSG);
    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY + 1);

    setup.claim_unbonded(NO_ERR_MSG);
    setup.assert_user_nft_balance(
        POOL2_TOKEN_ID,
        1,
        POOL2_QUANTITY_PER_NONCE as u64 - amount_to_stake as u64 + amount_to_unstake,
    );
}

#[test]
#[allow(deprecated)]
fn unbond_multiple_sft_nonces_includes_quantity() {
    let amount_to_stake = 5;
    let amount_to_unstake = 1;
    let score_per_unit = 5;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL2_TOKEN_ID, 1, amount_to_stake),
        new_nft_transfer(POOL2_TOKEN_ID, 2, amount_to_stake),
        new_nft_transfer(POOL2_TOKEN_ID, 3, amount_to_stake),
    ];
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::CodingDivisionSfts);
    setup.set_token_score(
        StakingModuleType::CodingDivisionSfts,
        POOL2_TOKEN_ID,
        score_per_unit,
    );
    setup.stake(&transfers, NO_ERR_MSG);

    setup.assert_user_score(
        StakingModuleType::CodingDivisionSfts,
        amount_to_stake * score_per_unit as u64,
    );

    setup.start_unbonding(
        POOL2_TOKEN_ID,
        &[
            (1, amount_to_unstake),
            (2, amount_to_unstake),
            (3, amount_to_unstake),
        ],
        NO_ERR_MSG,
    );
    setup
        .b_mock
        .set_block_timestamp(DEFAULT_UNBONDING_TIME_PENALTY + 1);

    setup.claim_unbonded(NO_ERR_MSG);
    setup.assert_user_nft_balance(
        POOL2_TOKEN_ID,
        1,
        POOL2_QUANTITY_PER_NONCE as u64 - amount_to_stake as u64 + amount_to_unstake,
    );
    setup.assert_user_nft_balance(
        POOL2_TOKEN_ID,
        2,
        POOL2_QUANTITY_PER_NONCE as u64 - amount_to_stake as u64 + amount_to_unstake,
    );
    setup.assert_user_nft_balance(
        POOL2_TOKEN_ID,
        3,
        POOL2_QUANTITY_PER_NONCE as u64 - amount_to_stake as u64 + amount_to_unstake,
    );
}
