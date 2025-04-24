use std::str::FromStr;
use tendermint_rpc::{
    client::{Client, CompatMode, HttpClient},
    endpoint::broadcast::tx_sync::Response,
    HttpClientUrl,
};

pub struct AbciClient {
    client: HttpClient,
}

impl Default for AbciClient {
    fn default() -> Self {
        Self::new("http://127.0.0.1:26657").unwrap()
    }
}

impl AbciClient {
    pub fn new(tendermint_rpc_url: impl AsRef<str>) -> Result<Self, AbciClientError> {
        let rpc_url = HttpClientUrl::from_str(tendermint_rpc_url.as_ref())
            .map_err(AbciClientError::InvalidURL)?;

        let client = HttpClient::builder(rpc_url)
            .compat_mode(CompatMode::V0_38)
            .build()
            .map_err(AbciClientError::Build)?;

        Ok(Self { client })
    }

    pub async fn broadcast_transaction(
        &self,
        transaction: Vec<u8>,
    ) -> Result<Response, AbciClientError> {
        self.client
            .broadcast_tx_sync(transaction)
            .await
            .map_err(AbciClientError::BroadcastTransaction)
    }
}

#[derive(Debug)]
pub enum AbciClientError {
    InvalidURL(tendermint_rpc::Error),
    Build(tendermint_rpc::Error),
    BroadcastTransaction(tendermint_rpc::Error),
}

impl std::fmt::Display for AbciClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AbciClientError {}
