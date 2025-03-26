use crate::types::*;

#[trait_variant::make(EthApi: Send)]
pub trait LocalEthApi: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + 'static;

    /// Returns a list of addresses owned by client.
    async fn accounts(&self) -> Result<Vec<Address>, Self::Error>;

    /// Introduced in EIP-4844, returns the current blob base fee in wei.
    async fn blob_base_fee(&self) -> Result<U256, Self::Error>;

    /// Returns the number of most recent block.
    async fn block_number(&self) -> Result<U256, Self::Error>;

    /// Executes a new message call immediately without creating a transaction on the block chain.
    async fn call(&self, parameter: EthCall) -> Result<Bytes, Self::Error>;

    /// Returns the chain ID of the current network.
    async fn chain_id(&self) -> Result<Option<U64>, Self::Error>;

    /// Returns the client coinbase address.
    async fn coinbase(&self) -> Result<Address, Self::Error>;

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
        parameter: EthCreateAccessList,
    ) -> Result<AccessListResult, Self::Error>;

    /// Generates and returns an estimate of how much gas is necessary to allow the transaction to
    /// complete.
    async fn estimate_gas(&self, parameter: EthEstimateGas) -> Result<U256, Self::Error>;

    /// Returns the Transaction fee history
    ///
    /// Introduced in EIP-1559 for getting information on the appropriate priority fee to use.
    ///
    /// Returns transaction base fee per gas and effective priority fee per gas for the
    /// requested/supported block range. The returned Fee history for the returned block range
    /// can be a subsection of the requested range if not all blocks are available.
    async fn fee_history(&self, parameter: EthFeeHistory) -> Result<FeeHistory, Self::Error>;

    /// Returns the current price per gas in wei.
    async fn gas_price(&self) -> Result<U256, Self::Error>;

    /// Returns the balance of the account of given address.
    async fn get_balance(&self, parameter: EthGetBalance) -> Result<U256, Self::Error>;

    /// Returns information about a block by hash.
    async fn get_block_by_hash(
        &self,
        parameter: EthGetBlockByHash,
    ) -> Result<Option<Block>, Self::Error>;

    /// Returns information about a block by number.
    async fn get_block_by_number(
        &self,
        parameter: EthGetBlockByNumber,
    ) -> Result<Option<Block>, Self::Error>;

    /// Returns all transaction receipts for a given block.
    async fn get_block_receipts(
        &self,
        parameter: EthBlockReceipts,
    ) -> Result<Option<Vec<TransactionReceipt>>, Self::Error>;

    /// Returns the number of transactions in a block from a block matching the given block hash.
    async fn get_block_transaction_count_by_hash(
        &self,
        parameter: EthGetBlockTransactionCountByHash,
    ) -> Result<Option<U256>, Self::Error>;

    /// Returns the number of transactions in a block matching the given block number.
    async fn get_block_transaction_count_by_number(
        &self,
        parameter: EthGetBlockTransactionCountByNumber,
    ) -> Result<Option<U256>, Self::Error>;

    /// Returns code at a given address at given block number.
    async fn get_code(&self, parameter: EthGetCode) -> Result<Bytes, Self::Error>;

    /// Returns the account and storage values of the specified account including the Merkle-proof.
    /// This call can be used to verify that the data you are pulling from is not tampered with.
    async fn get_proof(
        &self,
        parameter: EthGetProof,
    ) -> Result<EIP1186AccountProofResponse, Self::Error>;

    /// Returns the value from a storage position at a given address
    async fn get_storage_at(&self, parameter: EthGetStorageAt) -> Result<B256, Self::Error>;

    /// Returns information about a transaction by block hash and transaction index position.
    async fn get_transaction_by_block_hash_and_index(
        &self,
        parameter: EthGetTransactionByBlockHashAndIndex,
    ) -> Result<Option<Transaction>, Self::Error>;

    /// Returns information about a transaction by block number and transaction index position.
    async fn get_transaction_by_block_number_and_index(
        &self,
        parameter: EthGetTransactionByBlockNumberAndIndex,
    ) -> Result<Option<Transaction>, Self::Error>;

    /// Returns the information about a transaction requested by transaction hash.
    async fn get_transaction_by_hash(
        &self,
        parameter: EthgetTransactionByHash,
    ) -> Result<Option<Transaction>, Self::Error>;

    /// Returns the number of transactions sent from an address at given block number.
    async fn get_transaction_count(
        &self,
        parameter: EthGetTransactionCount,
    ) -> Result<U256, Self::Error>;

    /// Returns the receipt of a transaction by transaction hash.
    async fn get_transaction_receipt(
        &self,
        parameter: EthGetTransactionReceipt,
    ) -> Result<Option<TransactionReceipt>, Self::Error>;

    /// Returns the number of uncles in a block from a block matching the given block hash.
    async fn get_uncle_count_by_block_hash(
        &self,
        parameter: EthGetUncleCountByBlockHash,
    ) -> Result<Option<U256>, Self::Error>;

    /// Returns the number of uncles in a block with given block number.
    async fn get_uncle_count_by_block_number(
        &self,
        parameter: EthGetUncleCountByBlockNumber,
    ) -> Result<Option<U256>, Self::Error>;

    /// Introduced in EIP-1559, returns suggestion for the priority for dynamic fee transactions.
    async fn max_priority_fee_per_gas(&self) -> Result<U256, Self::Error>;

    /// Sends signed transaction, returning its hash.
    async fn send_raw_transaction(
        &self,
        parameter: EthSendRawTransaction,
    ) -> Result<B256, Self::Error>;

    /// Returns an Ethereum specific signature with: sign(keccak256("\x19Ethereum Signed Message:\n"
    /// + len(message) + message))).
    async fn sign(&self, parameter: EthSign) -> Result<Bytes, Self::Error>;

    /// Signs a transaction that can be submitted to the network at a later time using with
    /// `sendRawTransaction.`
    async fn sign_transaction(&self, parameter: EthSignTransaction) -> Result<Bytes, Self::Error>;

    /// Returns an object with data about the sync status or false.
    async fn syncing(&self) -> Result<SyncStatus, Self::Error>;
}
