use vm6502::prelude::{ StatusInterface, ProgramController}

#[proc_macro_derive(VMStatusI)]
fn impl_vm_status_interface(ast: &DeriveInput) -> TokenStream{
    let name = &ast.ident;

    quote ! {
        impl StatusInterface for #name {
            fn flip_status(&mut self, flag: Status) {
                let status = self.registers.sr;
        
                self.registers.sr = status ^ status!(flag);
            }
        
            fn set_status(&mut self, flag: Status, value: bool) {
                let status = self.registers.sr;
        
                if value {
                    self.registers.sr = status | status!(flag);
                } else {
                    self.registers.sr = status & !status!(flag);
                }
            }
        
            fn get_status(&self, flag: Status) -> bool {
                let status = self.registers.sr;
        
                status & status!(flag) != 0
            }
        
            fn reset_status(&mut self) {
                self.registers.sr = 0x00;
            }
        }
    }
}