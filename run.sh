#!/bin/bash

if ! dpkg -l | grep -q libgmp-dev; then
    echo "libgmp-dev is not installed. Installing..."
    sudo apt install libgmp-dev -y
fi

if ! dpkg -l | grep -q m4; then
    echo "m4 is not installed. Installing..."
    sudo apt install m4 -y
fi

if ! command -v hyperfine &> /dev/null; then
    echo "hyperfine is not installed. Installing..."
    cargo install hyperfine
fi

cargo build --release && hyperfine -w 2 -r 10 ./target/release/mersenne_primes