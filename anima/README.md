Anima uses maturin build tooling.

It's recommended you first create a virtual env in the root of emu6502 before installing.
# Setup


# Build
    
    maturin develop

or

    maturin release

# To develop Python:
After building:

    pip install -e .

S/N: maturin develop will rewrite this pip installation, useful for python only changes.

# Tests
    maturin tests

## Running the Demo Python Frontend
After building with maturin, you can