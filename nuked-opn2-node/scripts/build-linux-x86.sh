#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rustup target add i686-unknown-linux-gnu

$SCRIPT_DIR/build.sh build:linux:x86 linux-x86
