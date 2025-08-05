use anyhow::Result;
use clap::Parser;
use tracing::Level;

use mojave::{
    command::run_sequencer,
    logging::init_logging,
    options::Options,
    sequencer_options::SequencerOpts,
    version::get_version,
};

#[derive(Parser)]
#[command(name = "sequencer", author = "1six Technologies", version = get_version(), about = "Run a sequencer")]
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
    sequencer_opts: SequencerOpts,
}

#[tokio::main]
async fn main() -> Result<()> {
    let CLI {
        log_level,
        opts,
        sequencer_opts,
    } = CLI::parse();

    init_logging(log_level);

    run_sequencer(opts, sequencer_opts).await
}

