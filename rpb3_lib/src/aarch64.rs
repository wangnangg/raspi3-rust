use crate::utils::bit_range_get64;

pub fn read_current_el() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {0}, CurrentEL", out(reg) val);
    }

    return bit_range_get64(val, 3, 2);
}

pub fn read_sp() -> u64 {
    let val: u64;
    unsafe {
        asm!("mov {0}, sp", out(reg) val);
    }
    return val;
}

pub fn read_daif() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {0}, DAIF", out(reg) val);
    }
    return bit_range_get64(val, 9, 6);
}
