#!/bin/bash

set -e

docker build -t linux-x86-cross -f linux-x86-cross.Dockerfile .
docker build -t linux-x64-cross -f linux-x64-cross.Dockerfile .
