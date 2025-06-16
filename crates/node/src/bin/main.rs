use mohave_chain_node::MohaveChainNode;

#[tokio::main]
async fn main() {
    let handle = MohaveChainNode::init().await.unwrap();
    handle.await;
}
