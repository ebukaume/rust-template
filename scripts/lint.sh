#!/bin/sh

exec cargo clippy \
    -p docs \
    -p app
