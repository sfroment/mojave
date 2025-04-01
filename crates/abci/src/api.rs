use crate::types::*;

pub trait AbciApi {
    type Error: std::error::Error + Send + 'static;

    fn init_chain(&self, request: RequestInitChain) -> Result<ResponseInitChain, Self::Error>;

    fn check_tx(&self, request: RequestCheckTx) -> Result<ResponseCheckTx, Self::Error>;

    fn commit(&self, request: RequestCommit) -> Result<ResponseCommit, Self::Error>;

    fn prepare_proposal(
        &self,
        request: RequestPrepareProposal,
    ) -> Result<ResponsePrepareProposal, Self::Error>;

    fn process_proposal(
        &self,
        request: RequestProcessProposal,
    ) -> Result<ResponseProcessProposal, Self::Error>;
}
