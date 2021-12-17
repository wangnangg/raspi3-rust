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

fn init_uart(periph: &mut Peripherial) {
    periph.uart.regs.cntl.write(0);
    periph.uart.regs.lcr.write(3);
    periph.uart.regs.mcr.write(0);
    periph.uart.regs.ier.write(0);
    periph.uart.regs.iir.write(0xc6);
    periph.uart.regs.baud.write(270);

    periph.aux.enable_uart();
    set_mini_uart_tx(&mut periph.gpio, GPIOMiniUartTxPin::GPIO14);
    set_mini_uart_rx(&mut periph.gpio, GPIOMiniUartRxPin::GPIO15);
    periph.gpio.set_pull_state(&[14, 15], GPIOPullState::Off);
    periph.uart.enable_tx();
    periph.uart.enable_rx();
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
                rp = Reply::Write(Err(CommandError::OverwriteLoader { loader_end }));
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
            let loader_end = get_loader_end();
            if *addr <= loader_end {
                let rp = Reply::Jump(Err(CommandError::JumpInsideLoader { loader_end }));
                reply(&mut periph.uart, &UartReply::new(rp));
            } else {
                let rp = Reply::Jump(Ok(()));
                reply(&mut periph.uart, &UartReply::new(rp));
                display_code(&mut periph.gpio, LedCode::Jump as u32);
                let load_addr: u64 = *addr as u64;
                unsafe {
                    asm!("blr  {0}", in(reg) load_addr);
                }
            }
        }
    }
}

fn boot_signal(gpio: &mut GPIO) {
    for code in [0b1000, 0b0100, 0b0010, 0b0001, 0b0000] {
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
    init_uart(&mut periph);
    init_leds(&mut periph.gpio);

    //boot signal
    boot_signal(&mut periph.gpio);

    const BUFFER_SIZE: usize = 64;
    let mut cmd_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    const_assert!(size_of::<UartCommand>() <= BUFFER_SIZE);

    loop {
        display_code(&mut periph.gpio, LedCode::WaitMagic as u32);
        cmd_buffer[0] = periph.uart.recv();
        if !UartCommand::magic_match(cmd_buffer[0]) {
            continue;
        }
        display_code(&mut periph.gpio, LedCode::ReadCommand as u32);
        for p in 1..size_of::<UartCommand>() {
            cmd_buffer[p] = periph.uart.recv();
        }

        display_code(&mut periph.gpio, LedCode::DoCommand as u32);
        let uart_cmd: &UartCommand =
            unsafe { &*(&cmd_buffer[0] as *const u8 as *const UartCommand) };
        do_command(&mut periph, &uart_cmd.cmd);
    }
}
