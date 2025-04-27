use crate::backend::{error::BackendError, Backend};
use mandu_abci::types::Hash;
use mandu_rpc::api::eth::EthApi;
use mandu_types::{
    primitives::{Address, Bytes, B256, U256, U64},
    rpc::*,
};

impl EthApi for Backend {
    type Error = BackendError;

    /// Returns a list of addresses owned by client.
    async fn accounts(&self) -> Result<Vec<Address>, Self::Error> {
        Ok(Vec::default())
    }

    /// Introduced in EIP-4844, returns the current blob base fee in wei.
    async fn blob_base_fee(&self) -> Result<U256, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the number of most recent block.
    async fn block_number(&self) -> Result<U256, Self::Error> {
        let block_number = self.blockchain().read().await.get_current_number();
        Ok(U256::from(block_number))
    }

    /// Executes a new message call immediately without creating a transaction on the block chain.
    async fn call(&self, parameter: EthCall) -> Result<Bytes, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the chain ID of the current network.
    async fn chain_id(&self) -> Result<Option<U64>, Self::Error> {
        let chain_id = self.environments().read().await.cfg_env.chain_id;
        Ok(Some(U64::from(chain_id)))
    }

    /// Returns the client coinbase address.
    async fn coinbase(&self) -> Result<Address, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Generates an access list for a transaction.
    ///
    /// This method creates an [EIP2930](https://eips.ethereum.org/EIPS/eip-2930) type accessList based on a given Transaction.
    ///
    /// An access list contains all storage slots and addresses touched by the transaction, except
    /// for the sender account and the chain's precompiles.
    ///
    /// It returns list of addresses and storage keys used by the transaction, plus the gas
    /// consumed when the access list is added. That is, it gives you the list of addresses and
    /// storage keys that will be used by that transaction, plus the gas consumed if the access
    /// list is included. Like eth_estimateGas, this is an estimation; the list could change
    /// when the transaction is actually mined. Adding an accessList to your transaction does
    /// not necessary result in lower gas usage compared to a transaction without an access
    /// list.
    async fn create_access_list(
        &self,
        _parameter: EthCreateAccessList,
    ) -> Result<AccessListResult, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Generates and returns an estimate of how much gas is necessary to allow the transaction to
    /// complete.
    async fn estimate_gas(&self, _parameter: EthEstimateGas) -> Result<U256, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the Transaction fee history
    ///
    /// Introduced in EIP-1559 for getting information on the appropriate priority fee to use.
    ///
    /// Returns transaction base fee per gas and effective priority fee per gas for the
    /// requested/supported block range. The returned Fee history for the returned block range
    /// can be a subsection of the requested range if not all blocks are available.
    async fn fee_history(&self, _parameter: EthFeeHistory) -> Result<FeeHistory, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the current price per gas in wei.
    async fn gas_price(&self) -> Result<U256, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the balance of the account of given address.
    async fn get_balance(&self, parameter: EthGetBalance) -> Result<U256, Self::Error> {
        let blockchain = self.blockchain().read().await;

        // Parse the block ID to block hash and get the corresponding [`StateDatabase`].
        let state = match blockchain.get_block_hash_by_id(&parameter.block_number) {
            Some(block_hash) => blockchain
                .get_state(&block_hash)
                .ok_or(BackendError::InvalidBlockHash(block_hash))?,
            None => {
                let block_hash = blockchain.get_current_hash();
                blockchain
                    .get_state(&block_hash)
                    .ok_or(BackendError::InvalidBlockHash(block_hash))?
            }
        };

        // Release the lock.
        drop(blockchain);

        // Get the account balance.
        let account = state
            .get_account_info(parameter.address)
            .ok_or(BackendError::AccountDoesNotExist)?;
        Ok(account.balance)
    }

    /// Returns information about a block by hash.
    async fn get_block_by_hash(
        &self,
        parameter: EthGetBlockByHash,
    ) -> Result<Option<Block>, Self::Error> {
        let blockchain = self.blockchain().read().await;
        Ok(blockchain.get_block_by_hash(parameter.hash))
    }

    /// Returns information about a block by number.
    async fn get_block_by_number(
        &self,
        parameter: EthGetBlockByNumber,
    ) -> Result<Option<Block>, Self::Error> {
        let blockchain = self.blockchain().read().await;
        let block_hash = blockchain
            .get_block_hash_by_number_or_tag(&parameter.number)
            .ok_or(BackendError::InvalidBlockNumberOrTag(parameter.number))?;
        Ok(blockchain.get_block_by_hash(block_hash))
    }

    /// Returns all transaction receipts for a given block.
    async fn get_block_receipts(
        &self,
        parameter: EthBlockReceipts,
    ) -> Result<Option<Vec<TransactionReceipt>>, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the number of transactions in a block from a block matching the given block hash.
    async fn get_block_transaction_count_by_hash(
        &self,
        parameter: EthGetBlockTransactionCountByHash,
    ) -> Result<Option<U256>, Self::Error> {
        let blockchain = self.blockchain().read().await;
        match blockchain.get_block_by_hash(parameter.hash) {
            Some(block) => Ok(Some(U256::from(block.transactions.len()))),

            // Return [None] if the corresponding block does not exist.
            None => Ok(None),
        }
    }

    /// Returns the number of transactions in a block matching the given block number.
    async fn get_block_transaction_count_by_number(
        &self,
        parameter: EthGetBlockTransactionCountByNumber,
    ) -> Result<Option<U256>, Self::Error> {
        let blockchain = self.blockchain().read().await;
        match blockchain.get_block_hash_by_number_or_tag(&parameter.number) {
            Some(block_hash) => match blockchain.get_block_by_hash(block_hash) {
                Some(block) => Ok(Some(U256::from(block.transactions.len()))),

                // Return [None] if the corresponding block does not exist.
                None => Ok(None),
            },
            None => {
                // Release the lock.
                drop(blockchain);
                let len = self.transaction_pool().read().await.get_transaction_count();
                Ok(Some(U256::from(len)))
            }
        }
    }

    /// Returns code at a given address at given block number.
    async fn get_code(&self, parameter: EthGetCode) -> Result<Bytes, Self::Error> {
        let blockchain = self.blockchain().read().await;

        // Get the block hash by block ID.
        let block_hash = blockchain
            .get_block_hash_by_id(&parameter.block_number)
            .ok_or(BackendError::InvalidBlockId(parameter.block_number))?;

        // Get the corresponding [StateDatabase].
        let state = blockchain
            .get_state(&block_hash)
            .ok_or(BackendError::InvalidBlockHash(block_hash))?;

        // Release the lock.
        drop(blockchain);

        let code = state
            .get_account_info(parameter.address)
            .ok_or(BackendError::AccountDoesNotExist)?
            .code
            .ok_or(BackendError::CodeDoesNotExist)?;
        Ok(code.bytes())
    }

    /// Returns the account and storage values of the specified account including the Merkle-proof.
    /// This call can be used to verify that the data you are pulling from is not tampered with.
    async fn get_proof(
        &self,
        _parameter: EthGetProof,
    ) -> Result<EIP1186AccountProofResponse, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the value from a storage position at a given address
    async fn get_storage_at(&self, _parameter: EthGetStorageAt) -> Result<B256, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns information about a transaction by block hash and transaction index position.
    async fn get_transaction_by_block_hash_and_index(
        &self,
        parameter: EthGetTransactionByBlockHashAndIndex,
    ) -> Result<Option<Transaction>, Self::Error> {
        let blockchain = self.blockchain().read().await;

        // Get the block.
        let block = blockchain
            .get_block_by_hash(parameter.hash)
            .ok_or(BackendError::InvalidBlockHash(parameter.hash))?;

        // Release the lock.
        drop(blockchain);

        match block.transactions.as_transactions() {
            Some(transactions) => Ok(transactions.get(parameter.index.0).cloned()),

            // Return [None] if unable to find the transaction by index.
            None => Ok(None),
        }
    }

    /// Returns information about a transaction by block number and transaction index position.
    async fn get_transaction_by_block_number_and_index(
        &self,
        parameter: EthGetTransactionByBlockNumberAndIndex,
    ) -> Result<Option<Transaction>, Self::Error> {
        let blockchain = self.blockchain().read().await;

        match blockchain.get_block_hash_by_number_or_tag(&parameter.number) {
            Some(block_hash) => match blockchain.get_block_by_hash(block_hash) {
                Some(block) => match block.transactions.as_transactions() {
                    Some(transactions) => Ok(transactions.get(parameter.index.0).cloned()),

                    // Return [None] if unable to find the transaction by index.
                    None => Ok(None),
                },

                // Return [None] if the block number is invalid.
                None => Ok(None),
            },

            // Return [None] for requesting the pending block.
            None => Ok(None),
        }
    }

    /// Returns the information about a transaction requested by transaction hash.
    async fn get_transaction_by_hash(
        &self,
        parameter: EthgetTransactionByHash,
    ) -> Result<Option<Transaction>, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the number of transactions sent from an address at given block number.
    async fn get_transaction_count(
        &self,
        parameter: EthGetTransactionCount,
    ) -> Result<U256, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the receipt of a transaction by transaction hash.
    async fn get_transaction_receipt(
        &self,
        parameter: EthGetTransactionReceipt,
    ) -> Result<Option<TransactionReceipt>, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the number of uncles in a block from a block matching the given block hash.
    async fn get_uncle_count_by_block_hash(
        &self,
        parameter: EthGetUncleCountByBlockHash,
    ) -> Result<Option<U256>, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns the number of uncles in a block with given block number.
    async fn get_uncle_count_by_block_number(
        &self,
        parameter: EthGetUncleCountByBlockNumber,
    ) -> Result<Option<U256>, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Introduced in EIP-1559, returns suggestion for the priority for dynamic fee transactions.
    async fn max_priority_fee_per_gas(&self) -> Result<U256, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Sends signed transaction, returning its hash.
    async fn send_raw_transaction(
        &self,
        parameter: EthSendRawTransaction,
    ) -> Result<B256, Self::Error> {
        // Broacast the transaction.
        let response = self
            .abci_client()
            .broadcast_transaction(parameter.bytes.to_vec())
            .await
            .map_err(BackendError::BroadcastTransaction)?;

        if response.code.is_err() {
            return Err(BackendError::from(response.code.value()));
        }

        if response.hash.is_empty() {
            return Err(BackendError::InvalidTransactionHash);
        }

        let transaction_hash = B256::from_slice(response.hash.as_bytes());
        // // Notify pubsub service.
        // self.pubsub_service()
        //     .publish_pending_transaction(transaction_hash);
        Ok(transaction_hash)
    }

    /// Returns an Ethereum specific signature with: sign(keccak256("\x19Ethereum Signed Message:\n"
    /// + len(message) + message))).
    async fn sign(&self, _parameter: EthSign) -> Result<Bytes, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Signs a transaction that can be submitted to the network at a later time using with
    /// `sendRawTransaction.`
    async fn sign_transaction(&self, _parameter: EthSignTransaction) -> Result<Bytes, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns an object with data about the sync status or false.
    async fn syncing(&self) -> Result<SyncStatus, Self::Error> {
        Err(BackendError::Unimplemented)
    }
}
