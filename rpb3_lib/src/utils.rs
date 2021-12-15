use core::ptr::{read_volatile, write_volatile};

pub struct RW<T> {
    reg: T,
}

pub struct R<T> {
    reg: T,
}

pub struct W<T> {
    reg: T,
}

impl<T> R<T> {
    pub fn read(&self) -> T {
        unsafe { read_volatile(&self.reg) }
    }
}

impl<T> W<T> {
    pub fn write(&mut self, val: T) {
        unsafe {
            write_volatile(&mut self.reg, val);
        }
    }
}

impl<T> RW<T> {
    pub fn read(&self) -> T {
        unsafe { read_volatile(&self.reg) }
    }
    pub fn write(&mut self, val: T) {
        unsafe {
            write_volatile(&mut self.reg, val);
        }
    }
}

pub fn bit_range_mask32(msb: u32, lsb: u32) -> u32 {
    let width = 32;
    assert!(msb < width);
    assert!(lsb < width);
    assert!(msb >= lsb);
    let val = (!0u32) >> (width - 1 - msb);
    return ((!0u32) << lsb) & val;
}

pub fn bit_range_clear(val: u32, msb: u32, lsb: u32) -> u32 {
    val & (!bit_range_mask32(msb, lsb))
}

pub fn bit_range_set(old_val: u32, set_val: u32, msb: u32, lsb: u32) -> u32 {
    let cleared_val = bit_range_clear(old_val, msb, lsb);
    let set_mask = bit_range_mask32(msb - lsb, 0);
    let final_set_val = set_val & set_mask;
    assert_eq!(final_set_val, set_val);
    cleared_val | (final_set_val << lsb)
}

pub fn bit_range_get(val: u32, msb: u32, lsb: u32) -> u32 {
    (val & bit_range_mask32(msb, lsb)) >> lsb
}

pub fn delay(cycle_count: u64) {
    for _ in 0..cycle_count {
        unsafe {
            asm!("nop");
        }
    }
}

pub fn delay_us(us: u64) {
    delay(6 * us);
}

pub fn delay_ms(ms: u64) {
    delay_us(1000 * ms);
}

pub fn delay_s(s: u64) {
    delay_ms(1000 * s);
}
