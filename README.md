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

3. Initialize CometBFT configuration
```
cometbft init --home $COMETBFT_HOME_PATH
```

4. Run the chain:
```
./target/release/mandu-node $COMETBFT_HOME_PATH
```
