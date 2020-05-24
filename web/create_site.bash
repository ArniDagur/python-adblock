#!/bin/bash
set -euxo pipefail

# This script assumes that the current working directory is the repository
# root.

mkdir -p target
cp -r web/static target/github-pages
cargo doc --all-features
cp -r target/doc target/github-pages/docs
