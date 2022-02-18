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

# Copy images from inside container
mkdir -p images

rm -f -- images/CACHEDIR.TAG

cat > images/CACHEDIR.TAG<< EOF
Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by DOXA.
# For information about cache directory tags, see:
#	http://www.brynosaurus.com/cachedir/
EOF

docker cp "$image_id":/images/rootfs.img ./images/rootfs.img
docker cp "$image_id":/images/scratch.img ./images/scratch.img
docker cp "$image_id":/images/python_modules.img ./images/python_modules.img
