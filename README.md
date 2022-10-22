# emu6502
![t](https://github.com/GRAYgoose124/emu6502/actions/workflows/tests.yml/badge.svg)     ![t](https://github.com/GRAYgoose124/emu6502/actions/workflows/anima_tests.yml/badge.svg)

emu6502 is a full 6502 emulation, testing, and debugging suite.

## How to use   
    git clone git@github.com:GRAYgoose124/emu6502.git
    cd emu6502/

## Running & building the Rust frontend demo:
To run the project as a regular binary:

    cargo run --features=debug_vm

Or to build it as a standalone binary and run:

    cargo build --release
    ./target/release/emu65022



## Sub-crates: 
### Anima   !! in construction !!
Anima provides Python bindings for the virtual machine from crate::vm6502.

See it's README.md for more info. 

### vm6502
As the name suggests, this crate is the core 6502 virtual machine. Currently, if you'd like more information, please generate and check the docs with `cargo docs --open` and direct yourself to the vm6502 crate.