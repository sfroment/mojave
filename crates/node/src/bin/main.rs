use mandu_node::ManduNode;

#[tokio::main]
async fn main() {
    let handle = ManduNode::init().await.unwrap();
    handle.await;
}
