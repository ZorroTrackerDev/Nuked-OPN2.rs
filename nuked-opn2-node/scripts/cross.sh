#!/bin/bash

set -e

cd ..
cargo install cross
cross $@
cd -
