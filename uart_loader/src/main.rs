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
use rpb3_lib::uart::*;
use rpb3_lib::uart_command::*;
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

fn init_uart(periph: &mut Peripherial) {
    periph.aux.enable_uart();
    set_mini_uart_tx(&mut periph.gpio, GPIOMiniUartTxPin::GPIO14);
    set_mini_uart_rx(&mut periph.gpio, GPIOMiniUartRxPin::GPIO15);
    periph.gpio.set_pull_state(&[14, 15], GPIOPullState::Off);
    periph.uart.enable_tx();
    periph.uart.enable_rx();
}

fn uart_echo(periph: &mut Peripherial) {
    init_uart(periph);
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

extern "C" {
    //the location of this variable is provided by linker script. The value is meaningless.
    static __loader_end: u32;
}

fn get_loader_end() -> u32 {
    unsafe { &__loader_end as *const u32 as u32 }
}

fn reply(uart: &mut Uart, rp: &UartReply) {
    let ptr: *const u8 = rp as *const UartReply as *const u8;
    for offset in 0..size_of::<UartReply>() {
        let c = unsafe { *ptr.offset(offset as isize) };
        uart.send(c);
    }
}

fn do_command(periph: &mut Peripherial, cmd: &Command) {
    match cmd {
        Command::Write { size, addr } => {
            let loader_end = get_loader_end();
            let rp;
            if *addr <= loader_end {
                rp = Reply::Write(Err(WriteError::OverwriteLoader { loader_end }));
            } else {
                let dest: *mut u8 = (*addr) as *mut u8;
                for offset in 0..*size {
                    let c = periph.uart.recv();
                    unsafe {
                        *dest.offset(offset as isize) = c;
                    }
                }
                rp = Reply::Write(Ok(()));
            }
            reply(&mut periph.uart, &UartReply::new(rp));
        }
        Command::Jump { addr } => {
            let loader_end = unsafe { __loader_end };
            if *addr <= loader_end {
                let rp = Reply::Jump(Err(JumpError::JumpInsideLoader { loader_end }));
                reply(&mut periph.uart, &UartReply::new(rp));
            } else {
                let rp = Reply::Jump(Ok(()));
                reply(&mut periph.uart, &UartReply::new(rp));
                unsafe {
                    asm!("blr  {0}", in(reg) addr);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn rmain() -> ! {
    let mut periph = Peripherial::new();
    init_uart(&mut periph);

    //boot signal
    periph.gpio.set_gpio_func(16, GPIOFunc::Out);
    periph.gpio.set_pin_out(&[16]);

    const BUFFER_SIZE: usize = 64;
    let mut cmd_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    const_assert!(size_of::<UartCommand>() <= BUFFER_SIZE);

    loop {
        periph.gpio.set_pin_out(&[16]);
        cmd_buffer[0] = periph.uart.recv();
        if !UartCommand::magic_match(cmd_buffer[0]) {
            continue;
        }
        for p in 1..size_of::<UartCommand>() {
            cmd_buffer[p] = periph.uart.recv();
        }

        periph.gpio.clear_pin_out(&[16]);
        let uart_cmd: &UartCommand =
            unsafe { &*(&cmd_buffer[0] as *const u8 as *const UartCommand) };
        do_command(&mut periph, &uart_cmd.cmd);
        delay_s(1);
    }
}
