/// Re-export RPC types
pub use alloy::rpc::types::*;

use crate::{
    primitives::{Address, Bytes, B256, U64},
    serde_helpers::JsonStorageKey,
};
use serde::{Deserialize, Serialize};
use state::StateOverride;

#[derive(Debug, Deserialize, Serialize)]
pub struct EthCall {
    pub request: TransactionRequest,
    pub block_number: Option<BlockId>,
    pub state_overrides: Option<StateOverride>,
    pub block_overrides: Option<Box<BlockOverrides>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthCreateAccessList {
    pub request: TransactionRequest,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthEstimateGas {
    pub request: TransactionRequest,
    pub block_number: Option<BlockId>,
    pub state_override: Option<StateOverride>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthFeeHistory {
    pub block_count: U64,
    pub newest_block: BlockNumberOrTag,
    pub reward_percentiles: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBalance {
    pub address: Address,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockByHash {
    pub hash: B256,
    pub full: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockByNumber {
    pub number: BlockNumberOrTag,
    pub full: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthBlockReceipts {
    pub block_id: BlockId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockTransactionCountByHash {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockTransactionCountByNumber {
    pub number: BlockNumberOrTag,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetCode {
    pub address: Address,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetProof {
    pub address: Address,
    pub keys: Vec<JsonStorageKey>,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetStorageAt {
    pub address: Address,
    pub index: JsonStorageKey,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionByBlockHashAndIndex {
    pub hash: B256,
    pub index: Index,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionByBlockNumberAndIndex {
    pub number: BlockNumberOrTag,
    pub index: Index,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthgetTransactionByHash {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionCount {
    pub address: Address,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionReceipt {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetUncleCountByBlockHash {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetUncleCountByBlockNumber {
    pub number: BlockNumberOrTag,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction {
    pub bytes: Bytes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthSign {
    pub address: Address,
    pub message: Bytes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthSignTransaction {
    pub transaction: TransactionRequest,
}

pub type TransactionHash = B256;
