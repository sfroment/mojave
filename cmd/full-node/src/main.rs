use anyhow::Result;
use clap::Parser;
use tracing::Level;

use mojave::{
    command::run_full_node,
    full_node_options::FullNodeOptions,
    logging::init_logging,
    options::Options,
    version::get_version,
};

#[derive(Parser)]
#[command(name = "full-node", author = "1six Technologies", version = get_version(), about = "Run a full node")]
struct CLI {
    #[arg(
        long = "log.level",
        default_value_t = Level::INFO,
        value_name = "LOG_LEVEL",
        help = "The verbosity level used for logs.",
        long_help = "Possible values: info, debug, trace, warn, error",
        help_heading = "Node options"
    )]
    log_level: Level,

    #[command(flatten)]
    opts: Options,

    #[command(flatten)]
    full_node_opts: FullNodeOptions,
}

#[tokio::main]
async fn main() -> Result<()> {
    let CLI {
        log_level,
        opts,
        full_node_opts,
    } = CLI::parse();

    init_logging(log_level);

    run_full_node(opts, full_node_opts).await
}

