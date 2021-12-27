#!/bin/bash

set -e

cd "$(dirname "$0")"

ls

# Allocate more space than we need to be sure
dd if=/dev/zero of=rootfs.img bs=1M count=500
mkfs.ext4 rootfs.img

dd if=/dev/zero of=python_modules.img bs=1M count=500
mkfs.ext4 python_modules.img

mkdir rootfs

mount rootfs.img rootfs || { ls; exit 1; }

./alpine-make-rootfs --branch v3.15 --script-chroot --timezone "Europe/London" rootfs "$(pwd)/alpine-install.sh"

mkdir python_modules
mount python_modules.img python_modules

mv rootfs/usr/lib/python3.9/* python_modules/

umount rootfs
rm -rf rootfs

umount python_modules
rm -rf python_modules


sleep 1
ls

e2fsck -p -D -f ./python_modules.img
e2fsck -p -D -f ./rootfs.img

resize2fs -M python_modules.img
resize2fs rootfs.img 200M
 
echo 'Making scratch base'

dd if=/dev/zero of=scratch.img bs=1M count=5
mkfs.ext4 scratch.img

mkdir scratch
mount scratch.img scratch

mkdir scratch/agent
chown -R 1000:1000 scratch/agent
chmod -R 770 scratch/agent

mkdir scratch/output
chown -R 1000:1000 scratch/output
chmod -R 770 scratch/output

umount scratch
rm -rf scratch

e2fsck -p -D -f ./scratch.img
