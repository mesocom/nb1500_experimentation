//! This example shows how to use the UART to perform transfers using the
//! embedded-hal-nb traits.

#![no_std]
#![no_main]
#![forbid(unsafe_code)] // Guaranteed 100% safe Rust :)

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use arduino_mkrnb1500 as bsp;
use bsp::hal;
use bsp::pac;
use hal::nb;

use bsp::{entry, periph_alias, pin_alias};
use hal::clock::GenericClockController;
use hal::ehal_nb::serial::{Read, Write};
use hal::fugit::RateExtU32;

use pac::Peripherals;

use core::cell::Cell;
use critical_section::Mutex;

static MY_VALUE: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let mut pm = peripherals.pm;
    let pins = bsp::Pins::new(peripherals.port);

    // Take peripheral and pins
    // let uart_sercom = periph_alias!(peripherals.sercom3);
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

    // Split uart in rx + tx halves
    let (mut rx, mut tx) = uart.split();

    // Make buffers to store data to send/receive
    let mut rx_buffer = [0x00; 50];
    // let mut tx_buffer = [0x00; 50];
    let tx_buffer = [0x69, 0x12, 10];

    // For fun, store numbers from 0 to 49 in buffer
    // for (i, c) in tx_buffer.iter_mut().enumerate() {
    //     *c = i as u8;
    // }

    critical_section::with(|cs| {
        // This code runs within a critical section.

        // `cs` is a token that you can use to "prove" that to some API,
        // for example to a `Mutex`:
        MY_VALUE.borrow(cs).set(42);
    });

    loop {
        // Send data. We block on each byte, but we could also perform some tasks while
        // waiting for the byte to finish sending.
        for c in tx_buffer.iter() {
            nb::block!(tx.write(*c)).unwrap();
        }

        // Receive data. We block on each byte, but we could also perform some tasks
        // while waiting for the byte to finish sending.
        rx.flush_rx_buffer();
        for c in rx_buffer.iter_mut() {
            *c = nb::block!(rx.read()).unwrap();
        }
    }
}
