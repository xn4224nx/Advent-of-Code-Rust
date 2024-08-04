#!/bin/bash
# Time how long it takes the solution to be found

# Compile the program
cargo build -r

# Time the program
time cargo run -r
time cargo run -r
time cargo run -r
time cargo run -r
time cargo run -r

# Delete the compiled binaries
cargo clean
