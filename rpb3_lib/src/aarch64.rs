pub enum ExceptionLevel {
    EL0 = 0,
    EL1,
    EL2,
    EL3,
}

pub fn get_exception_level() -> ExceptionLevel {
    let el_reg: u64;
    unsafe {
        asm!("mrs {0}, CurrentEL", out(reg) el_reg);
    }
    let el = crate::utils::bit_range_get64(el_reg, 3, 2);
    return match el {
        0b00 => ExceptionLevel::EL0,
        0b01 => ExceptionLevel::EL1,
        0b10 => ExceptionLevel::EL2,
        0b11 => ExceptionLevel::EL3,
        _ => panic!(),
    };
}

pub fn read_sp() -> u64 {
    let sp_val: u64;
    unsafe {
        asm!("mov {0}, sp", out(reg) sp_val);
    }
    return sp_val;
}
