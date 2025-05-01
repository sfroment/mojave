use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let websocket_url = arguments.get(0).expect("Provide the websocket URL");

    let connection_detail = WsConnect::new(websocket_url);
    let provider = ProviderBuilder::new()
        .on_ws(connection_detail)
        .await
        .unwrap();

    let task_1 = tokio::spawn({
        let provider = provider.clone();
        let mut block_stream = provider.subscribe_blocks().await.unwrap().into_stream();

        async move {
            while let Some(block) = block_stream.next().await {
                println!("{:#?}", block);
            }
        }
    });

    let task_2 = tokio::spawn({
        let provider = provider.clone();
        let mut transaction_stream = provider
            .subscribe_pending_transactions()
            .await
            .unwrap()
            .into_stream();

        async move {
            while let Some(transaction_hash) = transaction_stream.next().await {
                println!("{:?}", transaction_hash);
            }
        }
    });

    task_1.await.unwrap();
    task_2.await.unwrap();
}
