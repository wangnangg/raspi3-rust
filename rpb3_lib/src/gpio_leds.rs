use gpio::*;
use utils::*;

const LED_COUNT: usize = 4;
pub const MAX_CODE: u32 = (1 << LED_COUNT) - 1;
const LED_PINS: [u32; LED_COUNT] = [6, 13, 19, 26];
pub fn init_leds(gpio: &mut GPIO) {
    for pin in LED_PINS {
        gpio.set_gpio_func(pin, GPIOFunc::Out);
    }
}

pub fn display_code(gpio: &mut GPIO, code: u32) {
    for bit in 0..LED_COUNT {
        if bit_range_get(code, bit as u32, bit as u32) > 0 {
            gpio.set_pin_out(&LED_PINS[bit..bit + 1]);
        } else {
            gpio.clear_pin_out(&LED_PINS[bit..bit + 1]);
        }
    }
}
