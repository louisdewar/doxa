#!/bin/sh

echo Hello world

# read x
# 
# echo You wrote $x
# 
# exec /bin/sh

export RUST_BACKTRACE=1
/sbin/vm_executor

echo VM executor exited $?
