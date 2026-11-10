# Counter

A minimal counter smart contract for NEAR, written in Rust with [`near-sdk`](https://crates.io/crates/near-sdk) 5.x.

It stores a single `i64` (starting at `0`) and exposes these methods:

| Method | Type | Description |
|---|---|---|
| `get_num() -> i64` | view | Returns the current value |
| `increment() -> i64` | change | Adds 1, logs `Increased number to <new value>` |
| `decrement() -> i64` | change | Subtracts 1, logs `Decreased number to <new value>` |
| `reset() -> i64` | change | Sets the value back to 0, logs `Reset number to 0` |

The change methods also return the new value, and use checked math: incrementing past
`i64::MAX` or decrementing below `i64::MIN` panics instead of overflowing.

## Prerequisites

```sh
# Rust toolchain with the wasm target
rustup target add wasm32-unknown-unknown

# cargo-near, used to build reproducibly-friendly wasm
cargo install cargo-near

# NEAR CLI (JavaScript version), used for the commands below
npm install -g near-cli
```

## Build and test

```sh
cd counter

# Run the unit tests
cargo test

# Build the wasm artifact
cargo near build non-reproducible-wasm
```

The build writes the artifact to `target/non-reproducible-wasm/counter.wasm`.

## Deploy to testnet

Replace `<your-account>` with your own testnet account throughout.

```sh
# 1. Create a sub-account for the contract
near create-account counter.<your-account>.testnet --useAccount <your-account>.testnet

# 2. Deploy the wasm and run the `new` initializer
near deploy counter.<your-account>.testnet \
  target/non-reproducible-wasm/counter.wasm \
  --initFunction new \
  --initArgs '{}'
```

If you are using `near-cli-rs` (`cargo near`'s `near` binary) instead of the JavaScript CLI, the
equivalent deploy command is:

```sh
near contract deploy counter.<your-account>.testnet \
  use-file target/non-reproducible-wasm/counter.wasm with-init-call new \
  json-args '{}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
  network-config testnet sign-with-keychain send
```

## Call the contract

```sh
# Read the current value (free, 0 gas)
near view counter.<your-account>.testnet get_num

# Add 1
near call counter.<your-account>.testnet increment --accountId <your-account>.testnet

# Subtract 1
near call counter.<your-account>.testnet decrement --accountId <your-account>.testnet

# Set the value back to 0
near call counter.<your-account>.testnet reset --accountId <your-account>.testnet
```

Each change call returns the new value and logs it, e.g. `Increased number to 1`. The logs are
included in the transaction outcome (`near call ... --printLogs`) and the returned value can be
inspected with `near tx-status <tx-hash> --accountId <your-account>.testnet`.
