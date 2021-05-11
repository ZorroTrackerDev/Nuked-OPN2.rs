#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
DIR=$SCRIPT_DIR/..

platform=$1
target=$2
artifact_output=$3

pattern_darwin="darwin"
pattern_arm64_linux="linux-arm64"
pattern_arm64_windows="win32-arm64"

cd $DIR

name=$(node $SCRIPT_DIR/get_name.js)
version=$(node $SCRIPT_DIR/get_version.js)
artifact_name=$DIR/lib/$name.node
final_name=$name-v$version-napi-v4-$platform
final_artifact_dir=$DIR/prebuilds-artifacts/$final_name

rm -rf $artifact_name || true

if [[ $platform =~ $pattern_arm64_windows ]]
then
    cargo build --release -p nuked-opn2-node --target $target
else
    $SCRIPT_DIR/cross.sh build --release -p nuked-opn2-node --target $target 
fi


mkdir -p $DIR/lib
cp $DIR/../target/$target/release/$artifact_output $artifact_name

if [[ $platform =~ $pattern_darwin ]]
then
    strip -S $artifact_name
elif [[ $platform =~ $pattern_arm64_linux ]]
then
    aarch64-linux-gnu-strip $artifact_name
elif [[ $platform =~ $pattern_arm64_windows ]]
then
    echo No strip needed
else 
    strip $artifact_name
fi

mkdir -p $final_artifact_dir/lib
mv $artifact_name $final_artifact_dir/lib
mkdir -p $DIR/prebuilds
tar -czvf $DIR/prebuilds/$final_name.tar.gz -C $DIR/prebuilds-artifacts/$final_name lib
