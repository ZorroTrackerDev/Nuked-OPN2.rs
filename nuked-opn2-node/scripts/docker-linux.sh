#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

docker build -t linux-x86-cross -f $SCRIPT_DIR/linux-x86-cross.Dockerfile $SCRIPT_DIR
docker build -t linux-x64-cross -f $SCRIPT_DIR/linux-x64-cross.Dockerfile $SCRIPT_DIR
docker build -t linux-arm64-cross -f $SCRIPT_DIR/linux-arm64-cross.Dockerfile $SCRIPT_DIR
