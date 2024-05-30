# cargo run -- --dev

if [ "$1" == "weights" ]; then
    cargo b --features runtime-benchmarks &&
    target/debug/node-template benchmark pallet --chain dev --pallet pallet_tags --extrinsic \* --steps=50 --repeat=20 --wasm-execution=compiled --output pallets/tags/src/weights.rs --template ./.maintain/frame-weight-template.hbs
    exit
fi
cargo test