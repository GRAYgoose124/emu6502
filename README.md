# emu6502 ![](https://github.com/GRAYgoose124/emu6502/actions/workflows/tests.yml/badge.svg) ![](https://img.shields.io/crates/v/emu6502.svg)

emu6502 is a full 6502 emulation, testing, and debugging suite.

It was designed without much knowledge of the 6502 specification. However, it is intended to be fully capable of supporting real binary roms, and thus full test coverage is planned to develop organically.

    Notice: Currently there are no frontend features, as the backend is still being developed, generating the core debugging library in the process.

It uses [vm6502](https://github.com/GRAYgoose124/vm6502) as a backend.  Also found on [crates.io](https://crates.io/crates/vm6502).

It provides a frontend binary [crate](https://crates.io/crates/emu6502).


## How to use   
```bash
# Clone the repo and enter it.
    > git clone git@github.com:GRAYgoose124/emu6502.git
    > cd emu6502/
```
## Running & building the Rust frontend demo:
```bash
# To run the project as a regular binary:

    > cargo run --features=debug_vm

# Or to build it as a standalone binary and run:

    > cargo build --release
    > ./target/release/emu650
```

# Anima Sub-crate ![](https://github.com/GRAYgoose124/emu6502/actions/workflows/anima_tests.yml/badge.svg) ![](https://img.shields.io/crates/v/anima6502.svg) 
Anima provides Python bindings for the virtual machine from crate::vm6502.

Anima uses maturin build tooling.

It's recommended you first create a virtual env in the root of emu6502 before installing.
## Setup
### Build
```bash
    > cd 6502
    > maturin [develop|release]     # develoop automatically installs

    # To develop the Anima backend python
    > pip install -e .

    # Tests
    > maturin test
```  
S/N: maturin develop will rewrite this pip installation, useful for python only changes.

##  Running the Demo Python Frontend
After building with maturin, you can simply run anima, or use it's bindings in python with anima._anima.
```bash
    > anima6502
```
### Using Python bindings after installing 
```python
from anima._anima import Animator

animator = Animator()
animator.do_program(0x0000, "690101690101")
```