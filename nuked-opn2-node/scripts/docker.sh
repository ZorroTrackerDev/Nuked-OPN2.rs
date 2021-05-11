#!/bin/bash

set -e

docker build -t cross-win32-x86 -f win32-x86-cross.Dockerfile .
