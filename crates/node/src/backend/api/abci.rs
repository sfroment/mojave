use crate::{
    backend::{error::BackendError, Backend},
    service::{AbciRequest, AbciResponse},
};
use mandu_abci::{api::AbciApi, types::*};

impl AbciApi for Backend {
    /// TODO: Validate the transaction (Signature, Nonce, Balance, ETC..).
    fn check_tx(&self, request: RequestCheckTx) -> ResponseCheckTx {
        let mut receiver = self.abci_service().send(self.clone(), request);

        // # Safety
        // Very unlikely that the sender drops before sending a response.
        match receiver.try_recv().unwrap() {
            Some(AbciResponse::CheckTx(response)) => response,
            _others => panic!("Failed to check the transaction"),
        }
    }

    fn finalize_block(&self, _request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        ResponseFinalizeBlock::default()
    }

    fn commit(&self) -> ResponseCommit {
        let mut receiver = self.abci_service().send(self.clone(), AbciRequest::Commit);

        // # Safety
        // Very unlikely that the sender drops before sending a response.
        match receiver.try_recv().unwrap() {
            Some(AbciResponse::Commit(response)) => response,
            // Unreachable
            _others => panic!("Failed to commit"),
        }
    }
}

impl Backend {
    pub async fn check_transaction(&self, request: RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::default()
    }

    // pub async fn request_finalize_block(&self, request)

    pub async fn do_commit(&self) -> ResponseCommit {
        self.driver().mine_one().await;
        ResponseCommit::default()
    }
}
