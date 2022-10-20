# emu6502
![t](https://github.com/GRAYgoose124/emu6502/actions/workflows/tests.yml/badge.svg)

## How to use   

    git clone git@github.com:GRAYgoose124/emu6502.git
    cd emu6502

To run the project as a regular binary:

    cargo run --features=debug_vm

Or to build it as a standalone binary and run:

    cargo build --release
    ./target/release/emu65022

To run the virtual cpu tests:

    cargo test -p vm6502


## References
[6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)

[6502 Addressing Modes](http://www.emulator101.com/6502-addressing-modes.html)