#!/bin/bash

set -e
cd "$(dirname "$0")"
rm climatehack_getting_started.zip || echo 'no previous zip file'

zip -r climatehack_getting_started.zip climatehack_getting_started
