#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

docker build -t win32-x86-cross -f $SCRIPT_DIR/win32-x86-cross.Dockerfile $SCRIPT_DIR
docker build -t win32-x64-cross -f $SCRIPT_DIR/win32-x64-cross.Dockerfile $SCRIPT_DIR
