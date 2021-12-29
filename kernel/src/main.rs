#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::my_test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;
extern crate rpb3_lib;
extern crate spin;

global_asm!(include_str!("start.s"));
mod exception;
mod panic;
#[cfg(test)]
mod test;

use core::cell::RefCell;
use core::sync::atomic::AtomicBool;

use rpb3_lib::aarch64;
use rpb3_lib::gpio::*;
use rpb3_lib::gpio_leds::*;
use rpb3_lib::uart::*;
use rpb3_lib::utils::*;
use rpb3_lib::*;
use spin::Mutex;
use spin::MutexGuard;

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

pub static PERIPH: Mutex<RefCell<Option<Peripherial>>> = Mutex::new(RefCell::new(None));

pub fn fill_buf(buf: &mut [u8], fill_byte: u8) {
    for ch in buf.iter_mut() {
        *ch = fill_byte;
    }
}

pub fn format_hex_into(buf: &mut [u8], num: u32) -> bool {
    if buf.len() < 1 {
        return false;
    }

    let digit: &'static [u8; 16] = b"0123456789abcdef";
    let mut buf_iter = buf.iter_mut().rev();
    let mut num = num;
    while let Some(ch) = buf_iter.next() {
        *ch = digit[(num & 0x0f) as usize];
        num >>= 4;
    }
    return true;
}

#[no_mangle]
pub extern "C" fn rmain() -> ! {
    let mut periph = Peripherial::new();
    display_code(
        &mut periph.gpio,
        (aarch64::read_current_el() as u32) | 0b1000,
    );
    delay_s(1);

    let mut fmt_buffer: [u8; 8] = [0; 8];
    periph.uart.send_str("daif: ");
    format_hex_into(&mut fmt_buffer, aarch64::read_daif() as u32);
    let fmt_str = core::str::from_utf8(&fmt_buffer).unwrap();
    periph.uart.send_str(fmt_str);
    periph.uart.send_str("\r\n");

    unsafe {
        asm!("brk 2");
    }
    uart_echo(&mut periph.uart);
    // let abool = AtomicBool::new(false);
    // let res = abool.compare_exchange(false, true, core::sync::atomic::Ordering::Acquire, core::sync::atomic::Ordering::Relaxed);
    // match res {
    //     Ok(true) => display_code(&mut periph.gpio, 0b0010),
    //     Ok(false) => display_code(&mut periph.gpio, 0b0110),
    //     Err(true) => display_code(&mut periph.gpio, 0b1100),
    //     Err(false) => display_code(&mut periph.gpio, 0b1110),
    // }
    // loop{
    //     display_code(&mut periph.gpio, 0b0000);
    // }
    // {
    //     let periph_cell = PERIPH.lock();
    //     let old = periph_cell.replace(Some(Peripherial::new()));
    //     assert!(1 == 2);
    // }
    // let periph_cell_lock: MutexGuard<RefCell<Option<Peripherial>>> = PERIPH.lock();
    // let periph_cell: &RefCell<Option<Peripherial>> = &*periph_cell_lock;
    // let mut periph_ref : core::cell::RefMut<Option<Peripherial>> = periph_cell.borrow_mut();
    // let periph_option : &mut Option<Peripherial> = &mut *periph_ref;
    // let periph: &mut Peripherial = periph_option.as_mut().unwrap();
    // *periph = Peripherial::new();

    // //boot signal
    // boot_signal(&mut periph.gpio);
    // periph.uart.send_str("hello from rusty raspberry pi 3 B+!\r\n");
    // uart_echo(&mut periph.uart);
}
