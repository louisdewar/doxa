#!/bin/sh

# This script runs inside the alpine rootfs during installation

PATH=/bin:/sbin:/usr/bin:/usr/sbin

cp /mnt/init.sh /sbin/init
cp /mnt/vm_executor /sbin/vm_executor
ln -s /usr/bin/python3.9 /usr/bin/python

apk add python3
apk add py3-scikit-learn

addgroup -S -g 1000 doxa
adduser -S -u 1000 -G doxa doxa

# Create output dir for competitions that use it
mkdir /output
chown doxa:doxa /output

echo "Done setup"
