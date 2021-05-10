#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
DIR=$SCRIPT_DIR/..

npm_script=$1
platform=$2

cd $DIR

name=$(node $SCRIPT_DIR/get_name.js)
version=$(node $SCRIPT_DIR/get_version.js)
artifact_name=$DIR/lib/$name.node
final_name=$name-$version-napi-v4-$platform
final_artifact_dir=$DIR/prebuilds-artifacts/$final_name

rm -rf $artifact_name || true
npm run $npm_script
strip $artifact_name

mkdir -p $final_artifact_dir/lib
mv $artifact_name $final_artifact_dir/lib
tar -czvf $DIR/prebuilds/$final_name.tar.gz -C $DIR/prebuilds-artifacts/$final_name lib
