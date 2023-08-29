use multiversx_sc::types::Address;
#[allow(deprecated)]
use multiversx_sc_scenario::whitebox_legacy::{BlockchainStateWrapper, ContractObjWrapper};
use multiversx_sc_scenario::DebugApi;
use multiversx_sc_scenario::{rust_biguint, testing_framework::TxResult};
use nft_staking::NftStakingContract;

use self::constants::{
    NO_ERR_MSG, POOL1_QUANTITY_PER_NONCE, POOL1_TOKEN_ID, POOL2_QUANTITY_PER_NONCE, POOL2_TOKEN_ID,
};
use self::types::{TransferAssetType, TransferAssetTypeParserVec};

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
                sc.init();
            })
            .assert_ok();

        Self::add_asset_balance(&mut b_mock, &user_address);

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

    fn add_asset_balance(b_mock: &mut BlockchainStateWrapper, address: &Address) {
        let pool_1_quantity = rust_biguint!(POOL1_QUANTITY_PER_NONCE);
        let pool_2_quantity = rust_biguint!(POOL2_QUANTITY_PER_NONCE);
        for i in 0..100 {
            b_mock.set_nft_balance(address, POOL1_TOKEN_ID, i, &pool_1_quantity, b"");
            b_mock.set_nft_balance(address, POOL2_TOKEN_ID, i, &pool_2_quantity, b"");
        }
    }

    fn assert_tx_result(tx_result: &TxResult, err_msg: &str) {
        if err_msg == NO_ERR_MSG {
            tx_result.assert_ok();
            return;
        }
        tx_result.assert_user_error(err_msg);
    }
}
