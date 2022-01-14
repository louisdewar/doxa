#!/bin/bash

# This script builds the rootfs.img for the VM used by doxa
# It only requires docker to run.
set -e

cd "$(dirname "$0")"

VM_DIR=$(pwd)
cd ../../
ROOT=$(pwd)

cd "$VM_DIR"

echo "Removing old containers (if they exist)"
docker rm -f doxa-rootfs-builder

echo "Removed...Now building"


docker build -f ./build/Dockerfile.rootfs -t doxa-rootfs-builder-img "$ROOT"

image_id=$(docker create doxa-rootfs-builder-img)

# Privileged because it has to do mount operations
# docker run --privileged --name doxa-rootfs-builder doxa-rootfs-builder-img

# Copy images from inside container
mkdir -p images
docker cp "$image_id":/images/rootfs.img ./images/rootfs.img
docker cp "$image_id":/images/scratch.img ./images/scratch.img
docker cp "$image_id":/images/python_modules.img ./images/python_modules.img
