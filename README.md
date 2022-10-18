## How to use   

    git clone git@github.com:GRAYgoose124/emu6502.git
    cd emu6502

To run the project as a regular binary:

    cargo run

Or to build it as a standalone binary and run:

    cargo build --release
    ./target/release/emu65022

To run the virtual cpu tests:

    cd vm6502
    cargo test
