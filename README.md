# Prerequisites

## Install Rust
https://www.rust-lang.org/tools/install

## Install CometBFT
https://docs.cometbft.com/v1.0/tutorials/install

## Build & Run
1. Clone the repository:
```
https://github.com/d-roak/rs-drip_chain
```

2. Build the chain:
```
cd rs-drip_chain
cargo build --release
```

3. Initialize CometBFT configuration:
```
cometbft init --home $COMETBFT_HOME_PATH
```

4. [Recommended] Change the `timeout_commit` field of `config.toml`:
```
# How long we wait after committing a block, before starting on the new
# height (this gives us a chance to receive some more precommits, even
# though we already have +2/3).
# Set to 0 if you want to make progress as soon as the node has all the precommits.
# Default to 1 second.
timeout_commit = "1s" 
```

6. Run the chain:
```
./target/release/drip-chain-node $COMETBFT_HOME_PATH
```
