#!/bin/bash

set -e

cd "$(dirname "$0")"

rm rootfs.img || echo "rootfs.img did not already exist"

# In case there was a failure the last time this was run
umount rootfs || echo "rootfs was not mounted"
rm -rf rootfs || echo "couldn't remove rootfs folder (may not have existed)"

# Allocate more space than we need to be sure
dd if=/dev/zero of=rootfs.img bs=1M count=500
mkfs.ext4 rootfs.img

mkdir rootfs

mount rootfs.img rootfs || { ls; exit 1; }

./alpine-make-rootfs --branch v3.15 --script-chroot --timezone "Europe/London" rootfs "$(pwd)/alpine-install.sh"

rm ./vm_executor

umount rootfs
rm -rf rootfs
