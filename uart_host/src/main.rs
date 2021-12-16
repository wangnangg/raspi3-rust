extern crate rpb3_lib;
extern crate serialport;

use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::time::Duration;
use std::vec::Vec;

use rpb3_lib::uart_command::*;

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    SerialError(serialport::Error),
    AddrError { loader_end: u32, load_addr: u32 },
    ProtocolError,
}
fn from_io_error(err: io::Error) -> Error {
    Error::IoError(err)
}
fn from_serial_error(err: serialport::Error) -> Error {
    Error::SerialError(err)
}

fn load_file(path: &str) -> Result<Vec<u8>, Error> {
    let mut f = fs::File::open(path).map_err(from_io_error)?;
    let mut res: Vec<u8> = Vec::new();
    f.read_to_end(&mut res).map_err(from_io_error)?;
    Ok(res)
}

fn write_all(serial: &mut dyn serialport::SerialPort, content: &[u8]) -> io::Result<()> {
    let mut offset = 0;
    while offset < content.len() {
        let w_size = serial.write(&content[offset..])?;
        offset += w_size;
    }
    Ok(())
}

fn write_to_device(
    serial: &mut dyn serialport::SerialPort,
    load_addr: u32,
    content: &[u8],
) -> Result<(), Error> {
    let cmd = UartCommand::new(Command::Write {
        size: content.len() as u32,
        addr: load_addr,
    });

    write_all(serial, cmd.byte_view()).map_err(from_io_error)?;
    write_all(serial, &content).map_err(from_io_error)?;

    const BUFFER_SIZE: usize = std::mem::size_of::<UartReply>();
    let mut reply_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    serial
        .read_exact(&mut reply_buffer)
        .map_err(from_io_error)?;
    assert!(UartReply::magic_match(reply_buffer[0]));
    let uart_reply = unsafe { &*(&reply_buffer as *const [u8; BUFFER_SIZE] as *const UartReply) };

    match &uart_reply.reply {
        Reply::Write(Ok(_)) => Ok(()),
        Reply::Write(Err(WriteError::OverwriteLoader { loader_end })) => Err(Error::AddrError {
            load_addr: load_addr,
            loader_end: *loader_end,
        }),
        _ => Err(Error::ProtocolError),
    }
}

fn main() -> Result<(), Error> {
    let bin_path = env::args().nth(1).expect("no binary path given");
    let file_content = load_file(&bin_path)?;

    let mut serial = serialport::new("/dev/ttyUSB0", 115200)
        .open()
        .map_err(from_serial_error)?;
    serial
        .set_timeout(Duration::from_secs(10))
        .map_err(from_serial_error)?;

    let load_addr = 0x90000;
    match write_to_device(serial.as_mut(), load_addr, &file_content) {
        Ok(_) => println!("write was successful"),
        Err(err) => return Err(err),
    }

    return Ok(());
}
