#!/bin/bash

# This script runs inside the docker rootfs builder VM
# This occurs after the rootfs has been generated
# The purpose is to take the rootfs and produce the images.

set -e

IMAGES_DIR="/images"
IMAGES_SRC_DIR="/image_src"

ROOTFS_SRC="$IMAGES_SRC_DIR/rootfs"
PYTHON_MODULES_SRC="$IMAGES_SRC_DIR/python_modules"
SCRATCH_SRC="$IMAGES_SRC_DIR/scratch"

ROOTFS_IMG="$IMAGES_DIR/rootfs.img"
PYTHON_MODULES_IMG="$IMAGES_DIR/python_modules.img"
SCRATCH_IMG="$IMAGES_DIR/scratch.img"

# == PREPARE SOURCES
mv "$ROOTFS_SRC/python_env" "$PYTHON_MODULES_SRC"

rm -rf "$ROOTFS_SRC/root/.cache"

# === SCRATCH SOURCES
mkdir "$SCRATCH_SRC"

cd "$SCRATCH_SRC"

mkdir ./agent
chown -R 1000:1000 ./agent
chmod -R 770 agent

mkdir output
chown -R 1000:1000 output
chmod -R 770 output

cd /

# == CREATE IMAGES

du -sh "$ROOTFS_SRC"
du -sh "$PYTHON_MODULES_SRC"
du -sh "$SCRATCH_SRC"

# === ROOTFS
echo "=== ROOTFS"
# Allocate more space than we need to be sure
dd if=/dev/zero of="$ROOTFS_IMG" bs=1M count=200
mkfs.ext4 -U random -d "$ROOTFS_SRC" "$ROOTFS_IMG"

# === PYTHON_MODULES
echo "=== PYTHON_MODULES"
dd if=/dev/zero of="$PYTHON_MODULES_IMG" bs=1M count=3300
mkfs.ext4 -U random -d "$PYTHON_MODULES_SRC" "$PYTHON_MODULES_IMG"
# Shrink image to exact size
# For some reason this isn't actually shrinking the image file (it says it's already shrunk)
resize2fs -M "$PYTHON_MODULES_IMG"

# === SCRATCH
echo "=== SCRATCH"
dd if=/dev/zero of="$SCRATCH_IMG" bs=1M count=10
mkfs.ext4 -U random -d "$SCRATCH_SRC" "$SCRATCH_IMG"

# == FSCK
e2fsck -p -D -f "$PYTHON_MODULES_IMG"
e2fsck -p -D -f "$ROOTFS_IMG"
e2fsck -p -D -f "$SCRATCH_IMG"

ls -lh "$IMAGES_DIR"
