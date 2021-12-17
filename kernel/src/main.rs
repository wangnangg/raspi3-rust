#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

extern crate rlibc;
extern crate rpb3_lib;

mod panic;

use rpb3_lib::gpio::*;
use rpb3_lib::gpio_leds::*;
use rpb3_lib::uart::*;
use rpb3_lib::utils::*;
use rpb3_lib::*;

global_asm!(include_str!("start.s"));

fn boot_signal(gpio: &mut GPIO) {
    for code in [0b1111, 0b1110, 0b1100, 0b1000, 0b0000] {
        display_code(gpio, code);
        delay_ms(100);
    }
}

fn uart_echo(uart: &mut Uart) -> ! {
    loop {
        let c = uart.recv();
        match c {
            b'\r' => uart.send_str("\r\n"),
            _ => uart.send(c),
        }
    }
}


#[no_mangle]
pub extern "C" fn rmain() -> ! {
    let mut periph = Peripherial::new();
    //boot signal
    boot_signal(&mut periph.gpio);
    periph.uart.send_str("hello from rusty raspberry pi 3 B+!\r\n");
    uart_echo(&mut periph.uart);
}
