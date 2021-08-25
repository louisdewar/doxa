#!/bin/sh

# This script runs inside the alpine rootfs during installation

echo 'Hello world'

PATH=/bin:/sbin:/usr/bin

cp /mnt/init.sh /sbin/init
cp /mnt/vm_executor /sbin/vm_executor

# install -D -m 755 /mnt/vm_executor /sbin/vm_executor

apk add python3

# /sbin/vm_executor
# 
# ls -l /sbin

# apk add socat

# apk add openrc
# apk add util-linux
# 
# # Set up a login terminal on the serial console (ttyS0):
# ln -s agetty /etc/init.d/agetty.ttyS0
# echo ttyS0 > /etc/securetty
# rc-update add agetty.ttyS0 default
# 
# # Make sure special file systems are mounted on boot:
# rc-update add devfs boot
# rc-update add procfs boot
# rc-update add sysfs boot

# apk add set
# /sbin/setup-alpine
