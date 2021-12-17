#!/bin/bash

# This script builds the rootfs.img for the VM used by doxa
# It only requires docker to run.

echo "Removing old containers (if they exist)"
docker rm -f doxa-rootfs-builder

echo "Removed...Now building"

set -e

cd "$(dirname "$0")"

root=../..

docker build -f ./build/Dockerfile.rootfs -t doxa-rootfs-builder-img $root

# Privileged because it has to do mount operations
docker run --privileged --name doxa-rootfs-builder doxa-rootfs-builder-img

# Copy rootfs from inside container
docker cp doxa-rootfs-builder:/app/rootfs.img ./rootfs.img

docker rm -f doxa-rootfs-builder
