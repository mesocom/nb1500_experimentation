#![no_std]
#![no_main]

use arduino_mkrnb1500 as bsp;
use bsp::entry;
use bsp::hal;
use bsp::pac;
use hal::clock::GenericClockController;
// use panic_halt as _;

use panic_semihosting as _;

use cortex_m_semihosting::debug;

use defmt::{debug, error, info, warn};
use defmt_rtt as _;

use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::sercom::uart::{self, BaudMode, Oversampling};

use arduino_mkrnb1500::{GsmUart, GsmUartPads, GsmUartRx, GsmUartSercom, GsmUartTx};

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

    let mut pm = peripherals.pm;

    let pins = bsp::Pins::new(peripherals.port); // Creates an alias for bsp::Pins
    let mut led = pins.d6.into_push_pull_output(); // Creates LED pin like pinMode led = OUTPUT;
    let mut delay = Delay::new(core.SYST, &mut clocks); // Creates a new delay instance out of the system timer

    debug!("PooPoo");

    let mut mod_pwr = pins.gsm_dtr.into_push_pull_output();
    mod_pwr.set_high().unwrap();
    delay.delay_ms(500u32);
    debug!("PooPoo1");

    // Take peripheral and pins
    // let uart_sercom = periph_alias!(peripherals.sercom3);
    let uart_sercom = peripherals.sercom2;
    let uart_rx: GsmUartRx = pins.gsm_rx.into();
    let uart_tx: GsmUartTx = pins.gsm_tx.into();
    debug!("PooPoo1.2");

    // panic!("OMG");
    // Setup UART peripheral
    let uart = test_gsm_uart(
        &mut clocks,
        115200.Hz(),
        uart_sercom,
        &mut pm,
        uart_rx,
        uart_tx,
    );

    debug!("PooPoo2");

    // Split uart in rx + tx halves
    let (mut rx, mut tx) = uart.split();

    // Make buffers to store data to send/receive
    let mut rx_buffer = [0x00; 50];
    // let mut tx_buffer = [0x00; 50];
    let tx_buffer = b"AT\n";

    debug!("BOutta write");
    for c in tx_buffer.iter() {
        nb::block!(tx.write(*c)).unwrap();
    }

    debug!("BOutta read");

    // Receive data. We block on each byte, but we could also perform some tasks
    // while waiting for the byte to finish sending.
    rx.flush_rx_buffer();
    for c in rx_buffer.iter_mut() {
        *c = nb::block!(rx.read()).unwrap();
    }

    loop {
        delay.delay_ms(500u32);
        led.set_high().unwrap();
        delay.delay_ms(200u32);
        led.set_low().unwrap();
        error!("Hello {:?}", rx_buffer);
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

use hal::time::Hertz;

fn test_gsm_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: GsmUartSercom,
    pm: &mut pac::Pm,
    uart_rx: impl Into<GsmUartRx>,
    uart_tx: impl Into<GsmUartTx>,
) -> GsmUart {
    let gclk0 = clocks.gclk0();
    debug!("UART CLOCK A");
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    debug!("UART CLOCK B");
    let baud = baud.into();
    debug!("UART CLOCK C");

    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    let t = clock.freq().raw();
    debug!("UART CLOCK D {}", t);

    let mode = BaudMode::Fractional(Oversampling::Bits16);
    debug!("UART CLOCK D2");

    let x = uart::Config::new(pm, sercom, pads, clock.freq()).baud(baud, mode);
    // .enable();

    debug!("UART CLOCK D 3");

    let x = x.enable();

    debug!("UART CLOCK E");
    x
}

// #[panic_handler] // built-in ("core") attribute
// fn core_panic(info: &core::panic::PanicInfo) -> ! {
//     error!("PANIC: {:?}", info); // e.g. using RTT
//                                  // reset()
//     loop {}
// }

// #[defmt::panic_handler] // defmt's attribute
// fn defmt_panic() -> ! {
//     // leave out the printing part here
//     loop {}
// }
