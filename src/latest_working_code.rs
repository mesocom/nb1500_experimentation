//! This example shows how to use the UART to perform transfers using the
//! embedded-hal-nb traits.

#![no_std]
#![no_main]
#![forbid(unsafe_code)] // Guaranteed 100% safe Rust :)

use defmt::{debug, error, info, warn};
use defmt_rtt as _;

use arduino_mkrnb1500 as bsp;
use bsp::hal;
use bsp::pac;
use hal::nb;

use bsp::{entry, periph_alias, pin_alias};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::ehal_nb::serial::{Read, Write};
use hal::fugit::RateExtU32;

use panic_halt as _;

use hal::pac::{CorePeripherals, Peripherals};

use core::cell::Cell;
use critical_section::Mutex;

use hal::prelude::*;

#[entry]
fn main() -> ! {
    // Initialize defmt for debug output
    // defmt_rtt::init();

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap(); // Similar ^ but something to do with interrupts?

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let mut pm = peripherals.pm;
    let pins = bsp::Pins::new(peripherals.port);

    // Take peripheral and pins
    let uart_sercom = peripherals.sercom2;
    let uart_rx = pins.gsm_rx;
    let uart_tx = pins.gsm_tx;

    // Setup UART peripheral
    let uart = bsp::gsm_uart(
        &mut clocks,
        115200.Hz(),
        uart_sercom,
        &mut pm,
        uart_rx,
        uart_tx,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks); // Creates a new delay instance out of the system timer

    // Split uart in rx + tx halves
    let (mut rx, mut tx) = uart.split();

    // Make buffers to store data to send/receive
    let mut rx_buffer = [0x00; 50];
    // let tx_buffer = [0x69, 0x12, 10];

    let mut led = pins.d6.into_push_pull_output(); // Creates LED pin like pinMode led = OUTPUT;

    // Print an initial debug message to indicate the start of the main loop
    error!("Starting main loop...");

    loop {
        error!("Starting main loop...");
        delay.delay_ms(500u32);
        led.set_high().unwrap();
        delay.delay_ms(200u32);
        led.set_low().unwrap();
    }

    // loop {
    //     // Send data. We block on each byte, but we could also perform some tasks while
    //     // waiting for the byte to finish sending.
    //     for c in tx_buffer.iter() {
    //         nb::block!(tx.write(*c)).unwrap();
    //     }

    //     // Print debug message indicating data has been sent
    //     debug!("Data sent: {:?}", tx_buffer);

    //     // Receive data. We block on each byte, but we could also perform some tasks
    //     // while waiting for the byte to finish sending.
    //     rx.flush_rx_buffer();
    //     for c in rx_buffer.iter_mut() {
    //         *c = nb::block!(rx.read()).unwrap();
    //     }

    //     // Print debug message with the received data
    //     debug!("Data received: {:?}", rx_buffer);
    // }
}
