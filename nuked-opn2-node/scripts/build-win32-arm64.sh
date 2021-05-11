#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add aarch64-pc-windows-msvc

$SCRIPT_DIR/build.sh win32-arm64 aarch64-pc-windows-msvc nuked_opn2_node.dll

