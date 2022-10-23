# Anima ![](https://github.com/GRAYgoose124/emu6502/actions/workflows/anima_tests.yml/badge.svg)
Anima provides Python bindings for the virtual machine from crate::vm6502. It's primary frontend is found in [emu6502](https://crates.io/crates/emu6502).

Anima uses maturin build tooling. 

    If you are not using anima with the emu6502 crate please consider this an extreme work in progress.

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


