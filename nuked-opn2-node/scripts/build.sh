#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
DIR=$SCRIPT_DIR/..

npm_script=$1
platform=$2
artifact_output=$3

cd $DIR

name=$(node $SCRIPT_DIR/get_name.js)
version=$(node $SCRIPT_DIR/get_version.js)
artifact_name=$DIR/lib/$name.node
final_name=$name-v$version-napi-v4-$platform
final_artifact_dir=$DIR/prebuilds-artifacts/$final_name

rm -rf $artifact_name || true
npm run $npm_script
mkdir -p $DIR/lib
cp $DIR/../target/$artifact_output $artifact_name
strip $artifact_name

mkdir -p $final_artifact_dir/lib
mv $artifact_name $final_artifact_dir/lib
mkdir -p $DIR/prebuilds
tar -czvf $DIR/prebuilds/$final_name.tar.gz -C $DIR/prebuilds-artifacts/$final_name lib
