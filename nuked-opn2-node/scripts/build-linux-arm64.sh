#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

$SCRIPT_DIR/build.sh linux-arm64 aarch64-unknown-linux-gnu libnuked_opn2_node.so

