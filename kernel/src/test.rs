use rpb3_lib::{gpio::GPIO, gpio_leds::display_code, utils::delay_ms, Peripherial};

#[test_case]
fn test1() {
    assert_eq!(1, 1);
}

pub fn my_test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

enum LedCode {
    TestStarted = 0b0010,
    TestFailed = 0b0100,
    TestPassed = 0b1000,
}

fn boot_signal(gpio: &mut GPIO) {
    for code in [0b1000, 0b0100, 0b0010, 0b0001, 0b0010, 0b0100] {
        display_code(gpio, code);
        delay_ms(100);
    }
}

#[no_mangle]
pub extern "C" fn rmain() -> ! {
    let mut periph = Peripherial::new();
    //boot signal
    boot_signal(&mut periph.gpio);
    display_code(&mut periph.gpio, LedCode::TestStarted as u32);
    periph
        .uart
        .send_str("test strated on raspberry pi 3 B+!\r\n");

    crate::test_main();
    periph.uart.send_str("test passed!\r\n");
    display_code(&mut periph.gpio, LedCode::TestPassed as u32);
    loop {}
}
