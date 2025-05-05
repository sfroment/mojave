use drip_chain_node::DRiPNode;

#[tokio::main]
async fn main() {
    let handle = DRiPNode::init().await.unwrap();
    handle.await;
}
