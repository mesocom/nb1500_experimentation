#![no_std]
#![no_main]

use arduino_mkrnb1500 as bsp;
use bsp::entry;
use bsp::hal;
use bsp::pac;
use hal::clock::GenericClockController;
use panic_halt as _;

use cortex_m_semihosting::debug;

use defmt::{debug, error, info, warn};
use defmt_rtt as _;

use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap(); // This line controls access to peripherals so multiple instances cannot exist (ie only one thing can control a pin at once)
    let core = CorePeripherals::take().unwrap(); // Similar ^ but something to do with interrupts?

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    ); // Inits clocks of system
    let pins = bsp::Pins::new(peripherals.port); // Creates an alias for bsp::Pins
    let mut led = pins.d6.into_push_pull_output(); // Creates LED pin like pinMode led = OUTPUT;
    let mut delay = Delay::new(core.SYST, &mut clocks); // Creates a new delay instance out of the system timer

    loop {
        delay.delay_ms(500u32);
        led.set_high().unwrap();
        delay.delay_ms(200u32);
        led.set_low().unwrap();
        error!("Hello");
        debug!("Penis");
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}
