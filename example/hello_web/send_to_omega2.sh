#!/usr/bin/env bash

cargo build --target=mipsel-unknown-linux-musl

rsync -a --progress target/mipsel-unknown-linux-musl/debug/hello_web root@omega-CC67.local:/tmp