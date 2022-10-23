# emu6502
![](https://github.com/GRAYgoose124/emu6502/actions/workflows/tests.yml/badge.svg)![](https://github.com/GRAYgoose124/emu6502/actions/workflows/anima_tests.yml/badge.svg)carg

emu6502 is a full 6502 emulation, testing, and debugging suite.

## How to use   
    > git clone git@github.com:GRAYgoose124/emu6502.git
    > cd emu6502/

## Running & building the Rust frontend demo:
To run the project as a regular binary:

    > cargo run --features=debug_vm

Or to build it as a standalone binary and run:

    > cargo build --release
    > ./target/release/emu65022

## Sub-crates: 
### Anima `!! in construction !!`
Anima provides Python bindings for the virtual machine from crate::vm6502.

Anima uses maturin build tooling.

It's recommended you first create a virtual env in the root of emu6502 before installing.
#### Setup
##### Build
    > maturin [develop|release]
##### To develop Python with the Anima backend:
    > pip install -e .

S/N: maturin develop will rewrite this pip installation, useful for python only changes.
#### Tests
    > maturin test
#### Running the Demo Python Frontend
After building with maturin, you can simply run anima:

    > anima

### vm6502
As the name suggests, this crate is the core 6502 virtual machine. Currently, if you'd like 
more information, please generate and check the docs with `cargo docs --open` and direct yourself 
to the vm6502 crate.

To run the virtual cpu tests, first cd to the `vm6502` directory then run:
    cargo test

#### Features
`pretty', 'debug', and 'ugly' debugging modes available to emu6502.

When testing vm6502, you can enable 'show_test_debug' to see any error outputs.

#### References
[6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)

[6502 Addressing Modes](http://www.emulator101.com/6502-addressing-modes.html)