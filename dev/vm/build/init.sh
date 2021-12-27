#!/bin/sh

echo "DOXA - presetup"

mount -t proc proc /proc
mount -t sysfs sysfs /sys

# This string is searched for by the recorder system to know when bootup is complete (for log truncating)
echo "DOXA INIT started"

export RUST_BACKTRACE=1
/sbin/vm_executor

echo VM executor exited $?
