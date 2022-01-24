#!/bin/bash

set -e

cd "$(dirname "$0")"

echo "This script is a convenient tool to compile the vm_executor bin (in debug mode) and then replace the binary in the rootfs.img with this one"
echo "This is useful when you are making changes to the vm_executor and don't want to have to go through the entire rootfs build process to see small changes."
echo "You need to have the x86_64-unknown-linux-musl target installed"

ROOTFS_IMG=$(pwd)/images/rootfs.img
ROOTFS_DIR=$(pwd)/images/rootfs

sudo umount "$ROOTFS_DIR" || echo 'Did not need to unmount rootfs'

mkdir -p "$ROOTFS_DIR"
sudo mount "$ROOTFS_IMG" "$ROOTFS_DIR"

# NOTE: this is currently built using MUSL despite the VMs being debian based, this is because of
# an issue where this builds with a too recent version of glibc
cd ../../
cargo build --bin vm_executor --target x86_64-unknown-linux-musl


sudo cp ./target/x86_64-unknown-linux-musl/debug/vm_executor "$ROOTFS_DIR/sbin/vm_executor"

sudo umount "$ROOTFS_IMG"
