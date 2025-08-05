use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::Result;
use ethrex::{
    initializers::{get_local_node_record, get_signer, init_blockchain, init_store},
    utils::{store_node_config_file, NodeConfigFile},
};
use ethrex_blockchain::BlockchainType;
use ethrex_common::types::ELASTICITY_MULTIPLIER;
use ethrex_p2p::network::peer_table;
use ethrex_vm::EvmEngine;
use mojave_block_builder::{BlockBuilder, BlockBuilderContext};
use mojave_chain_utils::resolve_datadir;
use mojave_networking::rpc::clients::mojave::Client;
use tokio::sync::Mutex;
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{
    full_node_options::FullNodeOptions,
    initializer::{
        get_local_p2p_node, init_full_node_rpc_api, init_metrics, init_rollup_store,
        init_sequencer_rpc_api,
    },
    options::Options,
    sequencer_options::SequencerOpts,
};

pub async fn run_full_node(opts: Options, full_node_opts: FullNodeOptions) -> Result<()> {
    if opts.evm == EvmEngine::REVM {
        panic!("Mojave doesn't support REVM, use LEVM instead.");
    }

    let data_dir = resolve_datadir(&opts.datadir);
    let rollup_store_dir = data_dir.clone() + "/rollup_store";

    let genesis = opts.network.get_genesis()?;
    let store = init_store(&data_dir, genesis).await;
    let rollup_store = init_rollup_store(&rollup_store_dir).await;

    let blockchain = init_blockchain(opts.evm, store.clone(), BlockchainType::L2);

    let signer = get_signer(&data_dir);

    let local_p2p_node = get_local_p2p_node(&opts, &signer);

    let local_node_record = Arc::new(Mutex::new(get_local_node_record(
        &data_dir,
        &local_p2p_node,
        &signer,
    )));

    let peer_table = peer_table(local_p2p_node.node_id());

    // TODO: Check every module starts properly.
    let tracker = TaskTracker::new();

    let cancel_token = CancellationToken::new();

    init_full_node_rpc_api(
        &opts,
        &full_node_opts,
        peer_table.clone(),
        local_p2p_node.clone(),
        local_node_record.lock().await.clone(),
        store.clone(),
        blockchain.clone(),
        cancel_token.clone(),
        tracker.clone(),
        rollup_store.clone(),
    )
    .await;

    // Initialize metrics if enabled
    if opts.metrics_enabled {
        init_metrics(&opts, tracker.clone());
    }

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Server shut down started...");
            let node_config_path = PathBuf::from(data_dir + "/node_config.json");
            tracing::info!("Storing config at {:?}...", node_config_path);
            cancel_token.cancel();
            let node_config = NodeConfigFile::new(peer_table, local_node_record.lock().await.clone()).await;
            store_node_config_file(node_config, node_config_path).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
            tracing::info!("Server shutting down!");
        }
    }

    Ok(())
}

pub async fn run_sequencer(opts: Options, sequencer_opts: SequencerOpts) -> Result<()> {
    if opts.evm == EvmEngine::REVM {
        panic!("Mojave doesn't support REVM, use LEVM instead.");
    }

    let data_dir = resolve_datadir(&opts.datadir);
    let rollup_store_dir = data_dir.clone() + "/rollup_store";

    let genesis = opts.network.get_genesis()?;
    let store = init_store(&data_dir, genesis.clone()).await;
    let rollup_store = init_rollup_store(&rollup_store_dir).await;

    let blockchain = init_blockchain(opts.evm, store.clone(), BlockchainType::L2);

    let signer = get_signer(&data_dir);

    let local_p2p_node = get_local_p2p_node(&opts, &signer);

    let local_node_record = Arc::new(Mutex::new(get_local_node_record(
        &data_dir,
        &local_p2p_node,
        &signer,
    )));

    let peer_table = peer_table(local_p2p_node.node_id());

    // TODO: Check every module starts properly.
    let tracker = TaskTracker::new();

    let cancel_token = CancellationToken::new();

    init_sequencer_rpc_api(
        &opts,
        &sequencer_opts,
        peer_table.clone(),
        local_p2p_node.clone(),
        local_node_record.lock().await.clone(),
        store.clone(),
        blockchain.clone(),
        cancel_token.clone(),
        tracker.clone(),
        rollup_store.clone(),
    )
    .await;

    // Spawn block builder.
    let block_builder_context = BlockBuilderContext::new(
        store.clone(),
        blockchain.clone(),
        rollup_store.clone(),
        genesis.coinbase,
        ELASTICITY_MULTIPLIER,
    );
    let block_builder = BlockBuilder::start(block_builder_context, 100);
    let addrs: Vec<String> = sequencer_opts
        .full_node_addresses
        .iter()
        .map(|addr| format!("http://{addr}"))
        .collect();
    let mojave_client = Client::new(&addrs)?;
    tokio::spawn(async move {
        loop {
            match block_builder.build_block().await {
                Ok(block) => mojave_client
                    .send_broadcast_block(&block)
                    .await
                    .unwrap_or_else(|error| tracing::error!("{}", error)),
                Err(error) => {
                    tracing::error!("Error {}", error);
                }
            }
            tokio::time::sleep(Duration::from_millis(sequencer_opts.block_time)).await;
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Server shut down started...");
            let node_config_path = PathBuf::from(data_dir + "/node_config.json");
            tracing::info!("Storing config at {:?}...", node_config_path);
            cancel_token.cancel();
            let node_config = NodeConfigFile::new(peer_table, local_node_record.lock().await.clone()).await;
            store_node_config_file(node_config, node_config_path).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
            tracing::info!("Server shutting down!");
        }
    }

    Ok(())
}

