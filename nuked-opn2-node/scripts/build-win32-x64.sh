#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add x86_64-pc-windows-gnu

$SCRIPT_DIR/build.sh build:win32:x64 win32-x64

