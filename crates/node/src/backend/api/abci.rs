use crate::backend::{error::BackendError, Backend};
use mandu_abci::{api::AbciApi, types::*};
use mandu_types::{
    consensus::TxEnvelope,
    eips::Decodable2718,
    primitives::U256,
    rpc::{Receipt, TransactionTrait},
};
use revm::{context::TxEnv, Context, ExecuteCommitEvm, MainBuilder, MainContext};

impl AbciApi for Backend {
    /// TODO: Validate the transaction (Signature, Nonce, Balance, ETC..).
    fn check_tx(&self, request: RequestCheckTx) -> ResponseCheckTx {
        let mut response = ResponseCheckTx::default();
        match self.check_transaction(request) {
            Ok(()) => response.code = 0,
            Err(error) => response.code = error.into(),
        }
        response
    }

    fn finalize_block(&self, request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        // Get environment variables.
        let env = self.environments().blocking_read();
        let cfg_env = env.cfg_env.clone();
        let block_env = env.block_env.clone();
        drop(env);

        // Get database as a mutable reference.
        let database = &mut *self.database().blocking_write();

        // Build an executor.
        let mut evm = Context::mainnet()
            .with_cfg(cfg_env)
            .with_block(block_env)
            .with_db(database)
            .build_mainnet();

        for data in request.txs {
            let tx_env = self.convert_transaction(data.as_ref());
            let receipt = match evm.transact_commit(tx_env) {
                Ok(receipt) => {}
                Err(error) => {}
            };
        }

        ResponseFinalizeBlock::default()
    }

    fn commit(&self) -> ResponseCommit {
        let state = self.database().blocking_read().clone();
        let blockchain = self.blockchain().blocking_write();
        // blockchain.insert_state(hash, state);
        ResponseCommit::default()
    }
}

impl Backend {
    pub fn check_transaction(&self, request: RequestCheckTx) -> Result<(), BackendError> {
        let mut data = request.tx.as_ref();

        // Check if the transaction is empty.
        if data.is_empty() {
            return Err(BackendError::EmptyRawTransaction);
        }

        // Check if the transaction is decodable.
        let transaction =
            TxEnvelope::decode_2718(&mut data).map_err(|_| BackendError::DecodeTransaction)?;

        // Get the current state.
        let blockchain = self.blockchain().blocking_read();
        let state = blockchain
            .get_current_state()
            .ok_or(BackendError::GetCurrentState)?;
        drop(blockchain);

        // Get the account.
        let address = transaction
            .recover_signer()
            .map_err(|_| BackendError::RecoverSigner)?;
        let account = state
            .get_account_info(address)
            .ok_or(BackendError::AccountDoesNotExist)?;

        // Check nonce.
        if account.nonce > transaction.nonce() {
            return Err(BackendError::NonceTooLow)?;
        }

        // Check balance.
        let gas_limit = transaction.gas_limit() as u128;
        let gas_price = transaction.gas_price().unwrap_or_default();
        let max_cost = gas_limit.saturating_mul(gas_price);
        let total_cost = transaction.value() + U256::from(max_cost);

        if account.balance < total_cost {
            return Err(BackendError::InsufficientBalance);
        }

        Ok(())
    }

    pub fn convert_transaction(&self, mut data: &[u8]) -> TxEnv {
        let transaction = TxEnvelope::decode_2718(&mut data).unwrap();
        TxEnv {
            tx_type: transaction.tx_type().into(),
            caller: transaction.recover_signer().unwrap(),
            gas_limit: transaction.gas_limit(),
            gas_price: transaction.gas_price().unwrap_or_default(),
            kind: transaction.kind(),
            value: transaction.value(),
            data: transaction.input().to_owned(),
            nonce: transaction.nonce(),
            chain_id: transaction.chain_id(),
            access_list: transaction.access_list().cloned().unwrap_or_default(),
            gas_priority_fee: None,
            blob_hashes: Vec::default(),
            max_fee_per_blob_gas: transaction.max_fee_per_blob_gas().unwrap_or_default(),
            authorization_list: transaction
                .authorization_list()
                .unwrap_or_default()
                .to_vec(),
        }
    }
}
