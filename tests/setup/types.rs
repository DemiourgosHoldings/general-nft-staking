use multiversx_sc_scenario::{rust_biguint, testing_framework::TxTokenTransfer};

pub type TransferAssetType<'a> = (&'a [u8], u64, u64);

pub fn new_nft_transfer<'a>(id: &'a [u8], nonce: u64, amount: u64) -> TransferAssetType<'a> {
    (id, nonce, amount)
}

pub trait TransferAssetTypeParser {
    fn parse(&self) -> TxTokenTransfer;
}

impl TransferAssetTypeParser for TransferAssetType<'_> {
    fn parse(&self) -> TxTokenTransfer {
        TxTokenTransfer {
            token_identifier: self.0.to_vec(),
            nonce: self.1.into(),
            value: rust_biguint!(self.2),
        }
    }
}

pub trait TransferAssetTypeParserVec {
    fn parse_vec(&self) -> Vec<TxTokenTransfer>;
}

impl TransferAssetTypeParserVec for Vec<TransferAssetType<'_>> {
    fn parse_vec(&self) -> Vec<TxTokenTransfer> {
        self.iter()
            .map(|transfer_asset_type| transfer_asset_type.parse())
            .collect()
    }
}
