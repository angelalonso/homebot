# First test on Rust
Goal: check that we can do the same as our previous script on python, but with Rust

# Steps:
TODO:
- Prepare laptop to cross compile
Run:
cargo build --features live --release --target=aarch64-unknown-linux-gnu

Copy over /target/aarch64-unknown-linux-gnu/release/homebot to the Raspberry Pi 
SSH to the machine and run:
./homebot

...and check that the LED blinks
