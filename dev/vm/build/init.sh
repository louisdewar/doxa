#!/bin/sh

# This script runs inside the VM at startup

echo "DOXA - presetup"

mount -t proc proc /proc
mount -t sysfs sysfs /sys

export RUST_BACKTRACE=1
/sbin/vm_executor vsock_listen --cid 2 --port 1001

echo VM executor exited $?
