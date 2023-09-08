use multiversx_sc::types::{Address, ManagedVec};
#[allow(deprecated)]
use multiversx_sc_scenario::whitebox_legacy::{BlockchainStateWrapper, ContractObjWrapper};
use multiversx_sc_scenario::{managed_address, managed_biguint, managed_token_id, DebugApi};
use multiversx_sc_scenario::{rust_biguint, testing_framework::TxResult};
use nft_staking::owner::OwnerModule;
use nft_staking::staking_modules::staking_module_type::StakingModuleType;
use nft_staking::storage::config::ConfigModule;
use nft_staking::storage::score::ScoreStorageModule;
use nft_staking::storage::user_data::UserDataStorageModule;
use nft_staking::types::start_unbonding_payload::StartUnbondingPayload;
use nft_staking::NftStakingContract;

use self::constants::{
    NO_ERR_MSG, POOL1_QUANTITY_PER_NONCE, POOL1_TOKEN_ID, POOL2_QUANTITY_PER_NONCE, POOL2_TOKEN_ID,
    REWARD_TOKEN_ID,
};
use self::types::{NonceQtyPair, TransferAssetType, TransferAssetTypeParserVec};
use nft_staking::types::nonce_qty_pair::NonceQtyPair as NonceQtyPairSc;

const WASM_PATH: &str = "../../output/nft-staking.wasm";
pub mod constants;
pub mod types;

#[allow(deprecated)]
pub struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> nft_staking::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<nft_staking::ContractObj<DebugApi>, ContractObjBuilder>,
}

#[allow(deprecated)]
impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> nft_staking::ContractObj<DebugApi>,
{
    pub fn new(contract_obj_builder: ContractObjBuilder) -> Self {
        let mut b_mock = BlockchainStateWrapper::new();
        let rust_zero = rust_biguint!(0u64);

        let owner_address = b_mock.create_user_account(&rust_zero);
        let user_address = b_mock.create_user_account(&rust_zero);

        let contract_wrapper = b_mock.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            contract_obj_builder,
            WASM_PATH,
        );

        b_mock
            .execute_tx(&owner_address, &contract_wrapper, &rust_zero, |sc| {
                sc.init(managed_token_id!(REWARD_TOKEN_ID));
                sc.base_asset_score(&managed_token_id!(POOL1_TOKEN_ID))
                    .set(1);
            })
            .assert_ok();

        b_mock.set_block_epoch(1);

        Self::add_asset_balance(&mut b_mock, &user_address, &owner_address);

        Self {
            b_mock,
            owner_address,
            user_address,
            contract_wrapper,
        }
    }

    pub fn stake(&mut self, transfers: &[TransferAssetType], err_msg: &str) {
        let parsed_transfers = transfers.to_vec().parse_vec();
        let tx_result = self.b_mock.execute_esdt_multi_transfer(
            &self.user_address,
            &self.contract_wrapper,
            &parsed_transfers,
            |sc| {
                sc.stake();
            },
        );
        Self::assert_tx_result(&tx_result, err_msg);
    }

    pub fn start_unbonding(
        &mut self,
        token_id: &[u8],
        nonce_qty_pairs: &[NonceQtyPair],
        err_msg: &str,
    ) {
        let tx_result = self.b_mock.execute_tx(
            &self.user_address,
            &self.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut nonce_qty_vec = ManagedVec::new();
                for nonce_qty_pair in nonce_qty_pairs {
                    nonce_qty_vec.push(NonceQtyPairSc {
                        nonce: nonce_qty_pair.0,
                        quantity: managed_biguint!(nonce_qty_pair.1),
                    });
                }
                let payload = StartUnbondingPayload {
                    token_identifier: managed_token_id!(token_id),
                    items: nonce_qty_vec,
                };

                sc.start_unbonding(payload);
            },
        );
        Self::assert_tx_result(&tx_result, err_msg);
    }

    pub fn claim_unbonded(&mut self, err_msg: &str) {
        let tx_result = self.b_mock.execute_tx(
            &self.user_address,
            &self.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_unbonded();
            },
        );
        Self::assert_tx_result(&tx_result, err_msg);
    }

    pub fn assert_user_score(&mut self, expected_score: u64) {
        let address = &self.user_address;
        self.b_mock
            .execute_query(&self.contract_wrapper, |sc| {
                let user_score = sc
                    .aggregated_user_staking_score(&managed_address!(address))
                    .get();
                assert_eq!(managed_biguint!(expected_score), user_score);
            })
            .assert_ok();
    }

    pub fn assert_secondary_user_score(
        &mut self,
        module_type: StakingModuleType,
        expected_score: u64,
    ) {
        let address = &self.user_address;
        self.b_mock
            .execute_query(&self.contract_wrapper, |sc| {
                let user_score = sc
                    .aggregated_user_secondary_staking_score(
                        &module_type,
                        &managed_address!(address),
                    )
                    .get();
                assert_eq!(managed_biguint!(expected_score), user_score);
            })
            .assert_ok();
    }

    pub fn set_token_score(&mut self, token_id: &[u8], score: usize) {
        self.b_mock
            .execute_tx(
                &self.owner_address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.base_asset_score(&managed_token_id!(token_id))
                        .set(&score);
                },
            )
            .assert_ok();
    }

    pub fn set_secondary_token_score(&mut self, token_id: &[u8], score: usize) {
        self.b_mock
            .execute_tx(
                &self.owner_address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.secondary_base_asset_score(&managed_token_id!(token_id))
                        .set(&score);
                },
            )
            .assert_ok();
    }

    pub fn set_token_nonce_score(&mut self, token_id: &[u8], nonce: u64, score: usize) {
        self.b_mock
            .execute_tx(
                &self.owner_address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.nonce_asset_score(&managed_token_id!(token_id), nonce)
                        .set(&score);
                },
            )
            .assert_ok();
    }

    pub fn set_full_set_score(&mut self, token_id: &[u8], score: usize) {
        self.b_mock
            .execute_tx(
                &self.owner_address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.full_set_score(&managed_token_id!(token_id)).set(&score);
                },
            )
            .assert_ok();
    }

    pub fn set_stake_pool_type(&mut self, token_id: &[u8], pool_type: StakingModuleType) {
        self.b_mock
            .execute_tx(
                &self.owner_address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.stake_pool_type_configuration(&managed_token_id!(token_id))
                        .set(&pool_type);
                },
            )
            .assert_ok();
    }

    pub fn distribute_reward(&mut self, amount: u64, err_msg: &str) {
        let tx_result = self.b_mock.execute_esdt_transfer(
            &self.owner_address,
            &self.contract_wrapper,
            REWARD_TOKEN_ID,
            0,
            &rust_biguint!(amount),
            |sc| {
                sc.distribute_reward();
            },
        );
        Self::assert_tx_result(&tx_result, err_msg);
    }

    pub fn assert_pending_reward(&mut self, expected_amount: u64) {
        let address = self.user_address.clone();
        self.b_mock
            .execute_query(&self.contract_wrapper, |sc| {
                //TODO: this will not work for all rewards
                let pending_rewards_vec = sc.get_pending_reward(managed_address!(&address));
                let pending_rewards = match pending_rewards_vec.is_empty() {
                    true => managed_biguint!(0),
                    false => pending_rewards_vec.get(0).amount,
                };

                assert_eq!(managed_biguint!(expected_amount), pending_rewards);
            })
            .assert_ok();
    }

    pub fn set_aggregated_score(&mut self, score: u64) {
        self.b_mock
            .execute_tx(
                &self.owner_address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.aggregated_staking_score().set(&managed_biguint!(score));
                },
            )
            .assert_ok();
    }

    pub fn assert_reward_rate(&mut self, epoch: u64, expected_amount: u64) {
        self.b_mock
            .execute_query(&self.contract_wrapper, |sc| {
                let reward_rate = sc
                    .reward_rate(epoch, &managed_token_id!(REWARD_TOKEN_ID))
                    .get();
                assert_eq!(managed_biguint!(expected_amount), reward_rate);
            })
            .assert_ok();
    }

    pub fn claim_rewards(&mut self, err_msg: &str) {
        let tx_result = self.b_mock.execute_tx(
            &self.user_address,
            &self.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_rewards();
            },
        );

        Self::assert_tx_result(&tx_result, err_msg);
    }

    pub fn assert_user_token_balance(
        &mut self,
        token_id: &[u8],
        token_nonce: u64,
        expected_balance: u64,
    ) {
        let balance = self
            .b_mock
            .get_esdt_balance(&self.user_address, token_id, token_nonce);
        assert_eq!(rust_biguint!(expected_balance), balance);
    }

    pub fn update_user_deb(&mut self, new_deb: u64) {
        let address = self.user_address.clone();
        self.b_mock
            .execute_tx(
                &self.owner_address,
                &self.contract_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.update_deb(managed_address!(&address), managed_biguint!(new_deb));
                },
            )
            .assert_ok();
    }

    pub fn assert_stored_rewards(&mut self, expected_amount: u64) {
        let address = self.user_address.clone();
        self.b_mock
            .execute_query(&self.contract_wrapper, |sc| {
                let stored_pending_rewards = sc
                    .pending_rewards(
                        &managed_address!(&address),
                        &managed_token_id!(REWARD_TOKEN_ID),
                    )
                    .get();
                assert_eq!(&managed_biguint!(expected_amount), &stored_pending_rewards);
            })
            .assert_ok();
    }

    pub fn assert_aggregated_score(&mut self, expected_score: u64) {
        self.b_mock
            .execute_query(&self.contract_wrapper, |sc| {
                let aggregated_score = sc.aggregated_staking_score().get();
                assert_eq!(managed_biguint!(expected_score), aggregated_score);
            })
            .assert_ok();
    }

    fn add_asset_balance(
        b_mock: &mut BlockchainStateWrapper,
        address: &Address,
        owner_address: &Address,
    ) {
        let pool_1_quantity = rust_biguint!(POOL1_QUANTITY_PER_NONCE);
        let pool_2_quantity = rust_biguint!(POOL2_QUANTITY_PER_NONCE);
        for i in 0..100 {
            b_mock.set_nft_balance(address, POOL1_TOKEN_ID, i, &pool_1_quantity, b"");
            b_mock.set_nft_balance(address, POOL2_TOKEN_ID, i, &pool_2_quantity, b"");
        }
        b_mock.set_esdt_balance(
            owner_address,
            REWARD_TOKEN_ID,
            &rust_biguint!(1_000_000_000),
        );
    }

    fn assert_tx_result(tx_result: &TxResult, err_msg: &str) {
        if err_msg == NO_ERR_MSG {
            tx_result.assert_ok();
            return;
        }
        tx_result.assert_user_error(err_msg);
    }
}
