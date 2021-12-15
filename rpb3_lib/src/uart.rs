use core::mem;
use utils::*;

#[repr(C)]
pub struct UartRegs {
    pub io: RW<u32>,      //mini uart i/o data
    pub ier: RW<u32>,     //mini uart interrupt enable
    pub iir: RW<u32>,     //mini uart interrupt identity
    pub lcr: RW<u32>,     //mini uart line control
    pub mcr: RW<u32>,     //mini uart modem control
    pub lsr: R<u32>,      //mini uart line status
    pub msr: R<u32>,      //mini uart modem status
    pub scratch: RW<u32>, //mini uart scratch
    pub cntl: RW<u32>,    //mini uart extra control
    pub stat: R<u32>,     //mini uart extra status
    pub baud: RW<u32>,    //mini uart baudrate
}

pub struct Uart {
    regs: &'static mut UartRegs,
}

impl Uart {
    pub fn new(base: u32) -> Uart {
        const_assert_eq!(mem::size_of::<UartRegs>(), 0x68 - 0x40 + 4);
        let uart = Uart {
            regs: unsafe { &mut *(base as *mut UartRegs) },
        };
        uart.regs.cntl.write(0);
        uart.regs.lcr.write(3);
        uart.regs.mcr.write(0);
        uart.regs.ier.write(0);
        uart.regs.iir.write(0xc6);
        uart.regs.baud.write(270);

        return uart;
    }

    pub fn enable_tx_rx(&mut self) {
        let val = self.regs.cntl.read();
        self.regs.cntl.write(bit_range_set(val, 0b11, 1, 0));
    }

    pub fn transmitter_empty(&self) -> bool {
        let val = self.regs.lsr.read();
        bit_range_get(val, 5, 5) > 0
    }

    pub fn send(&mut self, c: u8) {
        while !self.transmitter_empty() {}
        self.regs.io.write(c as u32);
    }

    pub fn send_str(&mut self, s: &str) {
        for c in s.as_bytes() {
            self.send(*c);
        }
    }

    pub fn data_ready(&self) -> bool {
        let val = self.regs.lsr.read();
        bit_range_get(val, 0, 0) > 0
    }

    pub fn recv(&mut self) -> u8 {
        while !self.data_ready() {}
        self.regs.io.read() as u8
    }
}
