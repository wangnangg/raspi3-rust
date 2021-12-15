use core::mem;
use utils::*;

#[repr(C)]
pub struct GPIORegs {
    gpfsel: [RW<u32>; 6], //function select
    reserved0: u32,
    gpset: [W<u32>; 2], //set
    reserved1: u32,
    gpclr: [W<u32>; 2], //clear
    reserved2: u32,
    gplev: [R<u32>; 2], //level
    reserved3: u32,
    gpeds: [RW<u32>; 2], //event detect status
    reserved4: u32,
    gpren: [RW<u32>; 2], //rising edge detect enable
    reserved5: u32,
    gpfen: [RW<u32>; 2], //falling edge detect enable
    reserved6: u32,
    gphen: [RW<u32>; 2], //high detect enable
    reserved7: u32,
    gplen: [RW<u32>; 2], //low detect enable
    reserved8: u32,
    gparen: [RW<u32>; 2], //async rising edge detect
    reserved9: u32,
    gpafen: [RW<u32>; 2], //async falling edge detect
    reserved10: u32,
    gppud: RW<u32>,         //pull-up/down enable
    gppudclk: [RW<u32>; 2], //pull-up/down enable clock
    reserved12: [u32; 4],
    test: RW<u32>,
}

pub struct GPIO {
    regs: &'static mut GPIORegs,
}

pub enum GPIOMiniUartTxPin {
    GPIO14 = 14,
    GPIO32 = 32,
    GPIO40 = 40,
}

pub enum GPIOMiniUartRxPin {
    GPIO15 = 15,
    GPIO33 = 33,
    GPIO41 = 41,
}

pub enum GPIOFunc {
    In = 0b000,
    Out = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

pub enum GPIOPullState {
    Off = 0b00,
    PullDown = 0b01,
    PullUp = 0b10,
}

impl GPIO {
    pub fn new(base: u32) -> GPIO {
        const_assert_eq!(mem::size_of::<GPIORegs>(), 0xB0 + 4);
        GPIO {
            regs: unsafe { &mut *(base as *mut GPIORegs) },
        }
    }

    pub fn set_gpio_func(&mut self, pin: u32, func: GPIOFunc) {
        assert!(pin <= 53);
        let fsel_index = (pin / 10) as usize;
        let fsel_offset = (pin % 10) * 3;
        let val = self.regs.gpfsel[fsel_index].read();
        let new_val = bit_range_set(val, func as u32, fsel_offset + 2, fsel_offset);
        self.regs.gpfsel[fsel_index].write(new_val);
    }

    pub fn set_mini_uart_tx(&mut self, pname: GPIOMiniUartTxPin) {
        let pin = pname as u32;
        self.set_gpio_func(pin, GPIOFunc::Alt5);
    }
    pub fn set_mini_uart_rx(&mut self, pname: GPIOMiniUartRxPin) {
        let pin = pname as u32;
        self.set_gpio_func(pin, GPIOFunc::Alt5);
    }

    pub fn set_pull_state(&mut self, pins: &[u32], state: GPIOPullState) {
        self.regs.gppud.write(state as u32);
        delay(150);
        let mut vals: [u32; 2] = [0, 0];
        for pin in pins {
            assert!(*pin <= 53);
            let pin_index = (*pin / 32) as usize;
            let pin_offset = (*pin % 32) as usize;
            vals[pin_index] |= 1 << pin_offset;
        }
        for i in 0..2 {
            self.regs.gppudclk[i].write(vals[i]);
        }
        delay(150);
        for i in 0..2 {
            self.regs.gppudclk[i].write(0);
        }
        self.regs.gppud.write(GPIOPullState::Off as u32);
    }

    pub fn set_pin_out(&mut self, pins: &[u32]) {
        let mut vals: [u32; 2] = [0, 0];
        for pin in pins {
            assert!(*pin <= 53);
            let pin_index = (*pin / 32) as usize;
            let pin_offset = (*pin % 32) as usize;
            vals[pin_index] |= 1 << pin_offset;
        }
        for i in 0..2 {
            self.regs.gpset[i].write(vals[i]);
        }
    }

    pub fn clear_pin_out(&mut self, pins: &[u32]) {
        let mut vals: [u32; 2] = [0, 0];
        for pin in pins {
            assert!(*pin <= 53);
            let pin_index = (*pin / 32) as usize;
            let pin_offset = (*pin % 32) as usize;
            vals[pin_index] |= 1 << pin_offset;
        }
        for i in 0..2 {
            self.regs.gpclr[i].write(vals[i]);
        }
    }
}

impl GPIO {}
