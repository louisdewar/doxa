#!/bin/bash

cd "$(dirname "$0")"

umount rootfs
rm rootfs.img
rm -rf rootfs

# Allocate more space than we need to be sure
dd if=/dev/zero of=rootfs.img bs=1M count=80
mkfs.ext4 rootfs.img

mkdir rootfs

mount rootfs.img rootfs

cp ../target/x86_64-unknown-linux-musl/release/vm_executor ./

./alpine-make-rootfs --branch v3.14 --script-chroot --timezone "Europe/London" rootfs $(pwd)/alpine-install.sh
# ./alpine-make-rootfs --branch v3.14 --script-chroot --timezone "Europe/London" rootfs.tar $(pwd)/alpine-install.sh

rm ./vm_executor

umount rootfs
rm -rf rootfs
