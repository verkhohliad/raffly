#!/bin/bash
set -e

cargo +stable build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/liquid_raffle.wasm ./liquid_raffle/res/
cp target/wasm32-unknown-unknown/release/raffle.wasm ./raffle/res/
cp target/wasm32-unknown-unknown/release/raffle_factory.wasm ./raffle_factory/res/
