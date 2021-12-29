#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Source {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}

use rpb3_lib::gpio_leds::*;
use rpb3_lib::*;
/// This function is called when an exception occurs. The `info` parameter
/// specifies the source and kind of exception that has occurred. The `esr` is
/// the value of the exception syndrome register. Finally, `tf` is a pointer to
/// the trap frame for the exception.
#[no_mangle]
pub extern "C" fn handle_exception(info: Info, esr: u32) {
    let mut periph = Peripherial::new();
    display_code(&mut periph.gpio, 0b1110);
    periph.uart.send_str("hello from exception handler\r\n");
    let vals = [
        info.source as u32,
        info.kind as u32,
        esr as u32,
        aarch64::read_daif() as u32,
        aarch64::read_current_el() as u32,
    ];
    for val in vals {
        let mut fmt_buffer: [u8; 8] = [0; 8];
        crate::format_hex_into(&mut fmt_buffer, val as u32);
        let fmt_str = core::str::from_utf8(&fmt_buffer).unwrap();
        periph.uart.send_str(fmt_str);
        periph.uart.send_str("\r\n");
    }
    loop {}
}
