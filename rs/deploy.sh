#!/bin/bash
if [ $(pwd) != $HOME/app/uxf/rs ]
then
    cd rs
fi
cargo build --release
cp -f $HOME/app/uxf/rs/target/release/uxf $HOME/opt/bin/uxf
