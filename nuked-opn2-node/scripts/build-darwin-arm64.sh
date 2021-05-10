#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add aarch64-apple-darwin

$SCRIPT_DIR/build.sh build:darwin:arm64 darwin-arm64

