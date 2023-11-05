#!/bin/sh

exec cargo watch \
    --quiet \
    --clear \
    -x \
    "run  -q"
