use std::time::Duration;
use pyo3::prelude::*;
use vm6502::prelude::*;

#[pyclass]
struct Animator {
    vm: VirtualMachine,
}

#[pymethods]
impl Animator {
    #[new]
    fn new() -> Self {
        Self {
            vm: VirtualMachine::new(),
        }
    }

    fn do_program(&mut self, offset: u16, prog: &str) {
        self.vm.set_program(offset, prog);
        println!("Loaded program and set PC appropriately:\n{}\n", prog);

        println!("VM state before execution: {:?}\n", self.vm);

        println!("Running program...");
        let (cycles, time) = self.vm.run(Duration::from_millis(1000));

        println!("\nFinal state: {:?}", self.vm);
        println!("\tHalt state: {:?}", self.vm.halted);
        println!(
            "\tCycles: 0x{:X}\t\tTotal time: {:?},   {:.1}C/s",
            cycles,
            time,
            cycles as f64 / time.as_secs_f64()
        );
    }
}

#[pymodule]
fn _anima(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Animator>()?;

    Ok(())
}

// Language: rust
// Path: anima/src/lib.rs
// Compare this snippet from vm6502/src/program.rs:
// 
// use crate::vm::prelude::*;
// 
// pub mod prelude {
//     pub use crate::program::ProgramController;
// }
// 
// pub trait ProgramController {
//     /// Insert a hex encoded string `prog` at heap offset `offset`.
//     fn insert_program(&mut self, offset: u16, prog: &str);
//     fn set_program(&mut self, offset: u16, prog: &str);
// 
//     /// Run the internal program.
//     fn execute(&mut self) -> u64;
// 
//     /// Run the internally set program at `offset` for `duration`.
//     fn run(&mut self, duration: Duration) -> (u64, Duration);
// 
//     /// Fill the stack with ops.
//     fn fill_stack(&mut self, ops: Vec<u8>);
// 
//     /// Reset machine state
