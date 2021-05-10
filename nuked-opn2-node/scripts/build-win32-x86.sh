#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add i686-pc-windows-gnu

$SCRIPT_DIR/build.sh build:win32:x86 win32-x86
