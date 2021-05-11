#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add x86_64-apple-darwin

$SCRIPT_DIR/build.sh build:darwin:x64 darwin-x64 x86_64-apple-darwin/release/libnuked_opn2_node.dylib

