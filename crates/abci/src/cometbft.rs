use std::path::PathBuf;

use tokio::process::{Child, Command};

async fn init(home_directory: &str) -> Result<(), std::io::Error> {
    let home_path = PathBuf::from(home_directory);
    let config_path = home_path.join("config").join("config.toml");

    let mut remove = Command::new("rm");
    let remove_handle = remove
        .args(["-rf", home_path.to_str().unwrap()])
        .kill_on_drop(true)
        .spawn()?;
    remove_handle.wait_with_output().await?;

    // Initialize the config.
    let mut cometbft = Command::new("cometbft");
    cometbft
        .args(["init", "--home", &home_path.to_str().unwrap()])
        .kill_on_drop(true)
        .spawn()?
        .wait_with_output()
        .await?;

    // Set "consensus.timeout_commit" = "5s"
    let mut cometbft = Command::new("cometbft");
    cometbft
        .args([
            "config",
            "--home",
            home_path.to_str().unwrap(),
            "set",
            config_path.to_str().unwrap(),
            "consensus.timeout_commit",
            "5s",
        ])
        .kill_on_drop(true)
        .spawn()?
        .wait_with_output()
        .await?;

    Ok(())
}

pub async fn start_tendermint_node(
    proxy_app_address: &str,
    home_directory: &str,
) -> Result<Child, std::io::Error> {
    init(home_directory).await?;

    let mut tendermint_node = Command::new("cometbft");
    tendermint_node.args([
        "start",
        "--proxy_app",
        &proxy_app_address,
        "--home",
        &home_directory,
    ]);

    let handle = tendermint_node.kill_on_drop(true).spawn()?;
    Ok(handle)
}
