extern crate rpb3_lib;
extern crate serialport;

use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::time::Duration;
use std::vec::Vec;

use rpb3_lib::uart_command::*;

fn load_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut res: Vec<u8> = Vec::new();
    let size = f.read_to_end(&mut res).expect("failed to read bin file");
    println!("file size: {0}", size);
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

fn main() -> Result<(), io::Error> {
    let bin_path = env::args().nth(1).expect("no binary path given");
    let file_content = match load_file(&bin_path) {
        Ok(vec) => vec,
        Err(err) => return Err(err),
    };

    let load_addr = 0x90000;
    let cmd = UartCommand::new(Command::Write {
        size: file_content.len() as u32,
        addr: load_addr,
    });

    let mut serial = serialport::new("/dev/ttyUSB0", 115200)
        .open()
        .expect("failed to open ttyUSB0");
    serial.set_timeout(Duration::from_secs(10))?;
    println!("write command");
    write_all(serial.as_mut(), cmd.byte_view())?;
    println!("write file");
    write_all(serial.as_mut(), &file_content)?;
    println!("read reply");

    const BUFFER_SIZE: usize = std::mem::size_of::<UartReply>();
    let mut reply_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    serial.read_exact(&mut reply_buffer)?;
    assert!(UartReply::magic_match(reply_buffer[0]));
    println!("parse reply");

    let uart_reply = unsafe { &*(&reply_buffer as *const [u8; BUFFER_SIZE] as *const UartReply) };
    match &uart_reply.reply {
        Reply::Write(Ok(_)) => println!("write was successful!"),
        Reply::Write(Err(WriteError::OverwriteLoader { loader_end })) => println!(
            "write overwrites loader: 0x{addr:x} <= 0x{loader_end:x}!",
            loader_end = loader_end,
            addr = load_addr
        ),
        _ => println!("invalid reply!"),
    }

    return Ok(());
}
