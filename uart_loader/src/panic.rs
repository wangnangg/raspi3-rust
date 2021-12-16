use core::panic::PanicInfo;
use rpb3_lib::gpio_leds::*;
use rpb3_lib::Peripherial;
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    let mut periph = Peripherial::new();
    display_code(&mut periph.gpio, MAX_CODE);
    loop {}
}
