use crate::types::*;

#[trait_variant::make(EthSubscriptionApi: Send)]
pub trait LocalEthSubscriptionApi {
    type Error: std::error::Error + Send + 'static;

    /// Create an ethereum subscription for the given params
    async fn subscribe(
        &self,
        kind: SubscriptionKind,
        params: Option<Params>,
    ) -> Result<(), Self::Error>;
}
