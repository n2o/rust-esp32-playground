// Based on https://github.com/esp-rs/esp-idf-hal/blob/master/examples/blinky.rs

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::sys::EspError;

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the
    // ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio48)?;

    loop {
        log::info!("LED on!");
        led.set_high()?;

        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);

        log::info!("LED off!");
        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }
}
