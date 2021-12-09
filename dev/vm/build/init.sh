#!/bin/sh

echo INIT started

export RUST_BACKTRACE=1
/sbin/vm_executor

echo VM executor exited $?
