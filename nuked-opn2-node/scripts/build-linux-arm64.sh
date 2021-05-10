#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add aarch64-unknown-linux-gnu

$SCRIPT_DIR/build.sh build:linux:arm64 linux-arm64

