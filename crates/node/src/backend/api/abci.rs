use crate::{
    backend::{error::BackendError, Backend},
    service::{AbciRequest, AbciResponse},
};
use mandu_abci::{api::AbciApi, types::*};

impl AbciApi for Backend {
    /// TODO: Validate the transaction (Signature, Nonce, Balance, ETC..).
    fn check_tx(&self, request: RequestCheckTx) -> ResponseCheckTx {
        let receiver = self.abci_service().send(self.clone(), request);

        // # Safety
        // Very unlikely that the sender drops before sending a response.
        match receiver.blocking_recv() {
            Ok(AbciResponse::CheckTx(response)) => response,
            _others => panic!("Failed to check the transaction"),
        }
    }

    fn finalize_block(&self, _request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        ResponseFinalizeBlock::default()
    }

    fn commit(&self) -> ResponseCommit {
        let receiver = self.abci_service().send(self.clone(), AbciRequest::Commit);

        // # Safety
        // Very unlikely that the sender drops before sending a response.
        match receiver.blocking_recv() {
            Ok(AbciResponse::Commit(response)) => response,
            _others => panic!("Failed to commit"),
        }
    }
}

impl Backend {
    pub async fn check_transaction(&self, request: RequestCheckTx) -> ResponseCheckTx {
        let mut response = ResponseCheckTx::default();
        let result = self
            .evm_client()
            .send_raw_transaction(request.tx.into())
            .await
            .map_err(BackendError::EthApi);

        match result {
            Ok(result) => {
                response.code = 0;
                response.data = result.to_vec().into();
            }
            Err(error) => {
                response.code = 1;
                response.log = error.to_string();
            }
        }
        response
    }

    pub async fn do_commit(&self) -> ResponseCommit {
        self.evm_client().mine_one().await;
        // # Safety
        // Block is guaranteed to exist as long as mine_one() succeeds.
        let block = self
            .evm_client()
            .block_by_number_full(mandu_types::rpc::BlockNumberOrTag::Latest)
            .await
            .unwrap();

        if let Some(block) = block {
            let full_block = block.into_inner();
            let new_head = full_block.header;
            self.pubsub_service().publish_new_head(new_head);
        }
        ResponseCommit::default()
    }
}
