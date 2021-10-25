#!/bin/bash
cd "$(dirname "$0")"

cargo depgraph --all-deps --focus $(ls ../crates | sed -z 's/\n/,/g') | dot -Tpng > depgraph.png
