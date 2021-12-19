#!/bin/bash

set -e
cd "$(dirname "$0")"
rm uttt_getting_started.zip || echo 'no previous zip file'

zip -r uttt_getting_started.zip uttt_getting_started
