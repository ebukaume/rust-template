#!/bin/sh

exec cargo clippy \
    --fix \
    --allow-dirty \
    -p server \
    -p app \
    -p migration \
    -p seed
    -- \
    --no-deps
    -D warnings \
