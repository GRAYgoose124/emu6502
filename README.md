# emu6502 ![](https://github.com/GRAYgoose124/emu6502/actions/workflows/tests.yml/badge.svg)

emu6502 is a full 6502 emulation, testing, and debugging suite.

It uses [vm6502](https://github.com/GRAYgoose124/vm6502) as a backend.
It also uses [anima6502](https://github.com/GRAYgoose124/anima6502) to utilize python bindings for the vm.

## How to use   
    > git clone git@github.com:GRAYgoose124/emu6502.git
    > cd emu6502/

## Running & building the Rust frontend demo:
To run the project as a regular binary:

    > cargo run --features=debug_vm

Or to build it as a standalone binary and run:

    > cargo build --release
    > ./target/release/emu65022