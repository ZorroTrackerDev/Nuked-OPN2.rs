#!/bin/bash

set -e

docker build -t win32-x86-cross -f win32-x86-cross.Dockerfile .
docker build -t win32-x64-cross -f win32-x64-cross.Dockerfile .
