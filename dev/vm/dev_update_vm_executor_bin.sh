#!/bin/bash

set -e

cd "$(dirname "$0")"

ROOTFS_IMG=$(pwd)/images/rootfs.img
ROOTFS_DIR=$(pwd)/images/rootfs

sudo umount "$ROOTFS_DIR" || echo 'Did not need to unmount rootfs'

mkdir -p "$ROOTFS_DIR"
sudo mount "$ROOTFS_IMG" "$ROOTFS_DIR"

cd ../../
cargo build --bin vm_executor --target x86_64-unknown-linux-musl


sudo cp ./target/x86_64-unknown-linux-musl/debug/vm_executor "$ROOTFS_DIR/sbin/vm_executor"

sudo umount "$ROOTFS_IMG"
