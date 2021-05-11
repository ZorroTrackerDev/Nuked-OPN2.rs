#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

$SCRIPT_DIR/build.sh build:linux:x64 linux-x64 x86_64-unknown-linux-gnu/release/libnuked_opn2_node.so

