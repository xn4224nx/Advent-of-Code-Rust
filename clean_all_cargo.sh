#!/bin/bash
# Clean all cargo projects beneath this folder
find . -name Cargo.toml -type f -execdir cargo clean \;
