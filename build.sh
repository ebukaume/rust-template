#!/bin/sh

exec cargo clippy \
    -p api_documentation \
    -p api_implementation && \
    cargo build \
    --release
