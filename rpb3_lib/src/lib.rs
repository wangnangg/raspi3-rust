#![no_std]
#![feature(asm)]

#[macro_use]
extern crate static_assertions;

pub mod aux;
pub mod gpio;
pub mod uart;
pub mod uart_command;
pub mod utils;

use aux::*;
use gpio::*;
use uart::*;

pub struct Peripherial {
    pub aux: Aux,
    pub uart: Uart,
    pub gpio: GPIO,
}

impl Peripherial {
    pub fn new() -> Peripherial {
        let periph_base = 0x3F000000;
        Peripherial {
            aux: Aux::new(periph_base + 0x215000),
            uart: Uart::new(periph_base + 0x215040),
            gpio: GPIO::new(periph_base + 0x200000),
        }
    }
}
