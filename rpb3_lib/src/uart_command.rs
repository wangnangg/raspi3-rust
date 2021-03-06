#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UartCommand {
    magic: u8,
    pub cmd: Command,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum Command {
    Write { size: u32, addr: u32 },
    Jump { addr: u32 },
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UartReply {
    magic: u8,
    pub reply: Reply,
}

impl UartReply {
    const MAGIC: u8 = 0x53;
    pub fn new(reply: Reply) -> UartReply {
        UartReply {
            magic: UartReply::MAGIC,
            reply: reply,
        }
    }
    pub fn magic_match(magic: u8) -> bool {
        magic == UartReply::MAGIC
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum CommandError {
    JumpInsideLoader { loader_end: u32 },
    OverwriteLoader { loader_end: u32 },
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum Reply {
    Write(Result<(), CommandError>),
    Jump(Result<(), CommandError>),
}

impl UartCommand {
    const MAGIC: u8 = 0x42;
    pub fn new(cmd: Command) -> UartCommand {
        UartCommand {
            magic: UartCommand::MAGIC,
            cmd: cmd,
        }
    }
    pub fn magic_match(magic: u8) -> bool {
        magic == UartCommand::MAGIC
    }

    pub fn byte_view(&self) -> &[u8] {
        let ptr = self as *const UartCommand as *const u8;
        return unsafe {
            &*core::ptr::slice_from_raw_parts(ptr, core::mem::size_of::<UartCommand>())
        };
    }
}
