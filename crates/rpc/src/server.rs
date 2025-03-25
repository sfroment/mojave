use crate::{api::eth::EthAPI, config::RpcConfig, types::*};
use hyper::Method;
use jsonrpsee::{
    core::RpcResult,
    server::{Server, ServerHandle},
    types::{ErrorCode, ErrorObjectOwned, Params},
    Extensions, RpcModule,
};
use std::{
    future::Future,
    marker::PhantomData,
    sync::Arc,
    task::{Context, Poll},
};
use tower_http::cors::{Any, CorsLayer};

pub struct RpcServer<T: EthAPI> {
    _node: PhantomData<T>,
}

impl<T: EthAPI> RpcServer<T> {
    pub async fn init(
        self,
        _config: &RpcConfig,
        node: T,
    ) -> Result<RpcServerHandle, RpcServerError> {
        let mut rpc_module = RpcModule::new(node);
        Self::register_eth_api(&mut rpc_module)?;

        let cors = CorsLayer::new()
            .allow_methods([Method::POST])
            .allow_origin(Any)
            .allow_headers([hyper::header::CONTENT_TYPE]);
        let cors_middleware = tower::ServiceBuilder::new().layer(cors);

        let server = Server::builder()
            .set_http_middleware(cors_middleware)
            // TODO: Get address from [`crate::config::RpcConfig`]
            .build("127.0.0.1:8545")
            .await
            .map_err(RpcServerError::Build)?;
        let server_handle = server.start(rpc_module);

        Ok(RpcServerHandle(Some(server_handle)))
    }
}

/// EthAPI implementations
impl<T: EthAPI> RpcServer<T> {
    fn register_eth_api(rpc_module: &mut RpcModule<T>) -> Result<(), RpcServerError> {
        rpc_module.register_async_method("eth_accounts", Self::accounts)?;
        rpc_module.register_async_method("eth_blobBaseFee", Self::blob_base_fee)?;
        rpc_module.register_async_method("eth_blockNumber", Self::block_number)?;
        rpc_module.register_async_method("eth_call", Self::call)?;
        rpc_module.register_async_method("eth_chainId", Self::chain_id)?;
        rpc_module.register_async_method("eth_coinbase", Self::coinbase)?;
        rpc_module.register_async_method("eth_createAccessList", Self::create_access_list)?;
        rpc_module.register_async_method("eth_estimateGas", Self::estimate_gas)?;
        rpc_module.register_async_method("eth_feeHistory", Self::fee_history)?;
        rpc_module.register_async_method("eth_gasPrice", Self::gas_price)?;
        rpc_module.register_async_method("eth_getBalance", Self::get_balance)?;
        rpc_module.register_async_method("eth_getBlockByHash", Self::get_block_by_hash)?;
        rpc_module.register_async_method("eth_getBlockByNumber", Self::get_block_by_number)?;
        rpc_module.register_async_method("eth_getBlockReceipts", Self::get_block_receipts)?;
        rpc_module.register_async_method(
            "eth_getBlockTransactionCountByNumber",
            Self::get_block_transaction_count_by_hash,
        )?;
        rpc_module.register_async_method(
            "eth_getBlockTransactionCountByNumber",
            Self::get_block_transaction_count_by_number,
        )?;
        rpc_module.register_async_method("eth_getCode", Self::get_code)?;
        rpc_module.register_async_method("eth_getProof", Self::get_proof)?;
        rpc_module.register_async_method("eth_getStorageAt", Self::get_storage_at)?;
        rpc_module.register_async_method(
            "eth_getTransactionByBlockHashAndIndex",
            Self::get_transaction_by_block_hash_and_index,
        )?;
        rpc_module.register_async_method(
            "eth_getTransactionByBlockNumberAndIndex",
            Self::get_transaction_by_block_number_and_index,
        )?;
        rpc_module
            .register_async_method("eth_getTransactionByHash", Self::get_transaction_by_hash)?;
        rpc_module.register_async_method("eth_getTransactionCount", Self::get_transaction_count)?;
        rpc_module
            .register_async_method("eth_getTransactionReceipt", Self::get_transaction_receipt)?;
        rpc_module.register_async_method(
            "eth_getUncleCountByBlockHash",
            Self::get_uncle_count_by_block_hash,
        )?;
        rpc_module.register_async_method(
            "eth_getUncleCountByBlockNumber",
            Self::get_uncle_count_by_block_number,
        )?;
        rpc_module
            .register_async_method("eth_maxPriorityFeePerGas", Self::max_priority_fee_per_gas)?;
        rpc_module.register_async_method("eth_sendRawTransaction", Self::send_raw_transaction)?;
        rpc_module.register_async_method("eth_sign", Self::sign)?;
        rpc_module.register_async_method("eth_signTransaction", Self::sign_transaction)?;
        rpc_module.register_async_method("eth_syncing", Self::syncing)?;
        Ok(())
    }

    /// Handler for [EthAPI::accounts]
    async fn accounts(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Vec<Address>> {
        context.accounts().await.into_rpc_result()
    }

    /// Handler for [EthAPI::blob_base_fee]
    async fn blob_base_fee(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        context.blob_base_fee().await.into_rpc_result()
    }

    /// Handler for [EthAPI::block_number]
    async fn block_number(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        context.block_number().await.into_rpc_result()
    }

    /// Handler for [EthAPI::call]
    async fn call(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Bytes> {
        let parameter = parameter.parse::<EthCall>()?;
        context.call(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::chain_id]
    async fn chain_id(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U64>> {
        context.chain_id().await.into_rpc_result()
    }

    /// Handler for [EthAPI::coinbase]
    async fn coinbase(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Address> {
        context.coinbase().await.into_rpc_result()
    }

    /// Handler for [EthAPI::create_access_list]
    async fn create_access_list(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<AccessListResult> {
        let parameter = parameter.parse::<EthCreateAccessList>()?;
        context
            .create_access_list(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::estimate_gas]
    async fn estimate_gas(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthEstimateGas>()?;
        context.estimate_gas(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::fee_history]
    async fn fee_history(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<FeeHistory> {
        let parameter = parameter.parse::<EthFeeHistory>()?;
        context.fee_history(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::gas_price]
    async fn gas_price(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        context.gas_price().await.into_rpc_result()
    }

    /// Handler for [EthAPI::get_balance]
    async fn get_balance(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthGetBalance>()?;
        context.get_balance(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::get_block_by_hash]
    async fn get_block_by_hash(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<Block>> {
        let parameter = parameter.parse()?;
        context.get_block_by_hash(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::get_block_by_number]
    async fn get_block_by_number(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<Block>> {
        let parameter = parameter.parse()?;
        context
            .get_block_by_number(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_block_receipts]
    async fn get_block_receipts(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<Vec<TransactionReceipt>>> {
        let parameter = parameter.parse::<EthBlockReceipts>()?;
        context
            .get_block_receipts(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_block_transaction_count_by_hash]
    async fn get_block_transaction_count_by_hash(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U256>> {
        let parameter = parameter.parse::<EthGetBlockTransactionCountByHash>()?;
        context
            .get_block_transaction_count_by_hash(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_block_transaction_count_by_number]
    async fn get_block_transaction_count_by_number(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U256>> {
        let parameter = parameter.parse::<EthGetBlockTransactionCountByNumber>()?;
        context
            .get_block_transaction_count_by_number(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_code]
    async fn get_code(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Bytes> {
        let parameter = parameter.parse::<EthGetCode>()?;
        context.get_code(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::get_proof]
    async fn get_proof(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<EIP1186AccountProofResponse> {
        let parameter = parameter.parse::<EthGetProof>()?;
        context.get_proof(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::get_storage_at]
    async fn get_storage_at(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<B256> {
        let parameter = parameter.parse::<EthGetStorageAt>()?;
        context.get_storage_at(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::get_transaction_by_block_hash_and_index]
    async fn get_transaction_by_block_hash_and_index(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<Transaction>> {
        let parameter = parameter.parse::<EthGetTransactionByBlockHashAndIndex>()?;
        context
            .get_transaction_by_block_hash_and_index(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_transaction_by_block_number_and_index]
    async fn get_transaction_by_block_number_and_index(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<Transaction>> {
        let parameter = parameter.parse::<EthGetTransactionByBlockNumberAndIndex>()?;
        context
            .get_transaction_by_block_number_and_index(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_transaction_by_hash]
    async fn get_transaction_by_hash(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<Transaction>> {
        let parameter = parameter.parse::<EthgetTransactionByHash>()?;
        context
            .get_transaction_by_hash(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_transaction_count]
    async fn get_transaction_count(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthGetTransactionCount>()?;
        context
            .get_transaction_count(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_transaction_receipt]
    async fn get_transaction_receipt(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<TransactionReceipt>> {
        let parameter = parameter.parse::<EthGetTransactionReceipt>()?;
        context
            .get_transaction_receipt(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_uncle_count_by_block_hash]
    async fn get_uncle_count_by_block_hash(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U256>> {
        let parameter = parameter.parse::<EthGetUncleCountByBlockHash>()?;
        context
            .get_uncle_count_by_block_hash(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::get_uncle_count_by_block_number]
    async fn get_uncle_count_by_block_number(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U256>> {
        let parameter = parameter.parse::<EthGetUncleCountByBlockNumber>()?;
        context
            .get_uncle_count_by_block_number(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::max_priority_fee_per_gas]
    async fn max_priority_fee_per_gas(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        context.max_priority_fee_per_gas().await.into_rpc_result()
    }

    /// Handler for [EthAPI::send_raw_transaction]
    async fn send_raw_transaction(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<B256> {
        let parameter = parameter.parse::<EthSendRawTransaction>()?;
        context
            .send_raw_transaction(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthAPI::sign]
    async fn sign(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Bytes> {
        let parameter = parameter.parse::<EthSign>()?;
        context.sign(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::sign_transaction]
    async fn sign_transaction(
        parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Bytes> {
        let parameter = parameter.parse::<EthSignTransaction>()?;
        context.sign_transaction(parameter).await.into_rpc_result()
    }

    /// Handler for [EthAPI::syncing]
    async fn syncing(
        _parameter: Params<'static>,
        context: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<SyncStatus> {
        context.syncing().await.into_rpc_result()
    }
}

/// Helper trait to convert [std::result::Result<T, E>] to [RpcResult<T>].
trait IntoRpcResult<T> {
    fn into_rpc_result(self) -> RpcResult<T>;
}

impl<T, E> IntoRpcResult<T> for Result<T, E>
where
    E: std::error::Error + Send + 'static,
{
    fn into_rpc_result(self) -> RpcResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ErrorObjectOwned::owned::<String>(
                ErrorCode::InternalError.code(),
                ErrorCode::InternalError.message(),
                Some(error.to_string()),
            )),
        }
    }
}

pub struct RpcServerHandle(Option<ServerHandle>);

impl Future for RpcServerHandle {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        loop {
            if let Some(handle) = this.0.take() {
                match handle.is_stopped() {
                    true => return Poll::Ready(()),
                    false => return Poll::Pending,
                }
            }
        }
    }
}

pub enum RpcServerError {
    Build(std::io::Error),
    RegisterMethod(jsonrpsee::core::RegisterMethodError),
}

impl From<jsonrpsee::core::RegisterMethodError> for RpcServerError {
    fn from(value: jsonrpsee::core::RegisterMethodError) -> Self {
        Self::RegisterMethod(value)
    }
}
