#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add aarch64-pc-windows-msvc

$SCRIPT_DIR/build.sh build:win32:arm64 win32-arm64

