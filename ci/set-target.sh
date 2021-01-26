#!/bin/sh

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 OS ARCHITECTURE" >&2
    exit 1
fi

os=$1
architecture=$2

output() {
    echo "Setting ACTIONS_RUST_TARGET=$1"
    echo "ACTIONS_RUST_TARGET=$1" >> "$GITHUB_ENV"
    exit 0
}

if [ "$architecture" = "x64" ]; then
    if [ "$os" = "ubuntu-latest" ]; then
        output "x86_64-unknown-linux-gnu"
    elif [ "$os" = "macos-latest" ]; then
        output "x86_64-apple-darwin"
    elif [ "$os" = "windows-latest" ]; then
        output "x86_64-pc-windows-msvc"
    else
        echo "Unknown 64-bit OS: $os"
        exit 1
    fi
elif [ "$architecture" = "x86" ]; then
    if [ "$os" = "ubuntu-latest" ]; then
        output "i686-unknown-linux-gnu"
    elif [ "$os" = "windows-latest" ]; then
        output "i686-pc-windows-msvc"
    else
        echo "Unknown 32-bit OS: $os"
        exit 1
    fi
else
    echo "Bad architecture: $architecture"
    exit 1
fi
