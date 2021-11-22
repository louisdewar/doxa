#!/bin/bash

cd "$(dirname "$0")"

cd ../crates/doxa_vm
cargo build --release --bin vm_executor --target x86_64-unknown-linux-musl
