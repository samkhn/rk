#!/bin/bash
#set -x

if [ -x "$(command -v cindex)" ]; then
	cindex .
fi

pushd $(dirname $0)

BUILD_DIR=$HOME/build/x/rs/

if [ ! -d "$BUILD_DIR" ]; then
	mkdir -p $BUILD_DIR
fi

cargo build --target-dir=$BUILD_DIR
cargo run --target-dir=$BUILD_DIR

popd
