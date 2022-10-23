# vm6502
As the name suggests, this crate is the core 6502 virtual machine. Currently, if you'd like 
more information, please generate and check the docs with `cargo docs --open` and direct yourself 
to the vm6502 crate.

To run the virtual cpu tests, first cd to the `vm6502` directory then run:
    cargo test

## Features
`pretty', 'debug', and 'ugly' debugging modes available to emu6502.

When testing vm6502, you can enable 'show_test_debug' to see any error outputs.

## References
[6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)

[6502 Addressing Modes](http://www.emulator101.com/6502-addressing-modes.html)