# Anima `!! in construction !!`
Anima provides Python bindings for the virtual machine from crate::vm6502.

Anima uses maturin build tooling.

It's recommended you first create a virtual env in the root of emu6502 before installing.
## Setup
### Build
    > maturin [develop|release]
### To develop Python with the Anima backend:
    > pip install -e .

S/N: maturin develop will rewrite this pip installation, useful for python only changes.
### Tests
    > maturin test
    
##  Running the Demo Python Frontend
After building with maturin, you can simply run anima:

    > anima
