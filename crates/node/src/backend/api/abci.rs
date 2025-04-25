use crate::backend::{error::BackendError, Backend};
use mandu_abci::{api::AbciApi, types::*};
use mandu_types::{consensus::TxEnvelope, eips::Decodable2718};

/// TODO: Set this value from config.
const MAX_TX_BYTES: usize = 1048576;

impl AbciApi for Backend {
    /// TODO: Validate the transaction (Signature, Nonce, Balance, ETC..).
    fn check_tx(&self, request: RequestCheckTx) -> ResponseCheckTx {
        match self.check_transaction(request) {}

        ResponseCheckTx::default()
    }

    fn finalize_block(&self, request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        ResponseFinalizeBlock::default()
    }

    fn commit(&self) -> ResponseCommit {
        // TODO: Execute EVM for transaction_commit().
        ResponseCommit::default()
    }
}

impl Backend {
    pub fn check_transaction(&self, request: RequestCheckTx) -> Result<TxEnvelope, BackendError> {
        let mut data = request.tx.as_ref();

        // Check if the transaction is empty.
        if data.is_empty() {
            return Err(BackendError::EmptyRawTransaction);
        }

        // Check if the transaction is decodable.
        let transaction =
            TxEnvelope::decode_2718(&mut data).map_err(|_| BackendError::DecodeTransaction)?;

        Ok(transaction)
    }
}
