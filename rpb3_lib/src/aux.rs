use core::mem;
use utils::*;

#[repr(C)]
pub struct AuxRegs {
    pub irq: R<u32>,      //aux interrupt status
    pub enables: RW<u32>, //aux enables
}

pub struct Aux {
    pub regs: &'static mut AuxRegs,
}

impl Aux {
    pub fn new(base: u32) -> Aux {
        const_assert_eq!(mem::size_of::<AuxRegs>(), 8);
        Aux {
            regs: unsafe { &mut *(base as *mut AuxRegs) },
        }
    }

    pub fn enable_uart(&mut self) {
        let val = self.regs.enables.read();
        self.regs.enables.write(val | 1);
    }
}
