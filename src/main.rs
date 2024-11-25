#![no_std]
#![no_main]

use arduino_mkrnb1500 as bsp;
use bsp::entry;
use bsp::hal;
use bsp::pac;
use hal::clock::GenericClockController;
use hal::usb::{usb_device::class_prelude::UsbBusAllocator, usb_device::prelude::*, UsbBus};
use panic_halt as _;
use usb_device::class_prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);

    // Configure the USB bus allocator
    // let usb_allocator: UsbBusAllocator<UsbBus> =
    //     UsbBus::new(&mut peripherals.pm, &mut clocks, peripherals.usb);

    let usb_allocator = bsp::usb_allocator(
        peripherals.usb,
        &mut clocks,
        &mut peripherals.pm,
        pins.usb_n,
        pins.usb_p,
    );

    // Create a serial port for USB communication
    let mut serial = SerialPort::new(&usb_allocator);

    // Configure the USB device
    let mut usb_device = UsbDeviceBuilder::new(&usb_allocator, UsbVidPid(0x2341, 0x8053))
        .strings(&[StringDescriptors::new(LangID::EN)
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .expect("Failed to set strings")
        .device_class(USB_CLASS_CDC)
        .build();

    let mut counter: u32 = 0;

    loop {
        // Poll the USB device
        if usb_device.poll(&mut [&mut serial]) {
            // Write a debug message over USB
            use core::fmt::Write;
            // let _ = write!(serial, "Debug: Counter = {}\r\n", counter);
            serial.write(b"Fuck you").unwrap();
            counter += 1;
        }
    }
}
