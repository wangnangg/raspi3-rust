#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

extern crate rlibc;
extern crate rpb3_lib;

use rpb3_lib::aux::*;
use rpb3_lib::gpio::*;
use rpb3_lib::uart::*;
use rpb3_lib::utils::*;
use rpb3_lib::*;

global_asm!(include_str!("start.s"));

fn blink_gpio16(periph: &mut Peripherial) {
    periph.gpio.set_gpio_func(16, GPIOFunc::Out);
    loop {
        periph.gpio.set_pin_out(&[16]);
        delay_s(1);
        periph.gpio.clear_pin_out(&[16]);
        delay_s(1);
    }
}

fn uart_echo(periph: &mut Peripherial) {
    periph.aux.enable_uart();
    periph.gpio.set_mini_uart_tx(GPIOMiniUartTxPin::GPIO14);
    periph.gpio.set_mini_uart_rx(GPIOMiniUartRxPin::GPIO15);
    periph.gpio.set_pull_state(&[14, 15], GPIOPullState::Off);
    periph.uart.enable_tx_rx();
    let uart = &mut periph.uart;
    uart.send(b'h');
    uart.send(b'!');
    uart.send(b'\n');
    loop {
        let c = uart.recv();
        match c {
            b'\r' => uart.send_str("\r\n"),
            _ => uart.send(c),
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let mut periph = Peripherial::new();

    blink_gpio16(&mut periph);
    return;
}
