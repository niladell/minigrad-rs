#!/bin/bash

# Build the Rust code and create the Python extension
maturin develop

# If maturin develop succeeds, run the Python test from the tests folder
if [ $? -eq 0 ]; then
    python examples/install_check.py
else
    echo "Build failed!"
fi
