#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

#[macro_use]
extern crate static_assertions;
extern crate rlibc;
extern crate rpb3_lib;

mod panic;

use core::mem::size_of;

use rpb3_lib::gpio::*;
use rpb3_lib::gpio_leds::*;
use rpb3_lib::uart::*;
use rpb3_lib::uart_command::*;
use rpb3_lib::utils::*;
use rpb3_lib::*;

global_asm!(include_str!("start.s"));

fn boot_signal(gpio: &mut GPIO) {
    for code in [0b1111, 0b1110, 0b1100, 0b1000, 0b0000] {
        display_code(gpio, code);
        delay_ms(500);
    }
}

enum LedCode {
    WaitMagic = 0b0001,
    ReadCommand = 0b0010,
    DoCommand = 0b0100,
    Jump = 0b1000,
}
#[no_mangle]
pub extern "C" fn rmain() -> ! {
    let mut periph = Peripherial::new();
    //boot signal
    boot_signal(&mut periph.gpio);

    assert!(1 == 2);

    loop {}
}
