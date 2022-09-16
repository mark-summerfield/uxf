#!/bin/bash
cd rs
cargo build --release
cp -f $HOME/app/uxf/rs/target/release/uxf $HOME/opt/bin/uxf
