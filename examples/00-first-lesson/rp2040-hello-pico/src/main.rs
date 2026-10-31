// A bare-metal firmware has no operating system or standard library.
#![no_std]
// The Cortex-M runtime supplies the entry point instead of a normal `main` ABI.
#![no_main]

// Halt the processor if an unrecoverable panic occurs.
use panic_halt as _;
// `entry` marks the reset entry point, `hal` provides board drivers, and `pac`
// exposes the RP2040 peripheral registers.
use rp_pico::{entry, hal, pac};
// USB device traits and the CDC serial class used by the host terminal.
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

// Firmware entry points never return, so the function returns the never type.
#[entry]
fn main() -> ! {
    // Take ownership of the device peripherals exactly once after reset.
    let mut peripherals = pac::Peripherals::take().unwrap();
    // The watchdog is required while the RP2040 clocks are configured.
    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);

    // Configure the Pico oscillator, system PLL, and USB PLL.
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Create the USB bus that owns the RP2040 USB peripheral and its DPRAM.
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        peripherals.USBCTRL_REGS,
        peripherals.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut peripherals.RESETS,
    ));

    // Add a USB CDC-ACM serial interface to the bus.
    let mut serial = SerialPort::new(&usb_bus);

    // Describe the USB device to the host computer.
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default().product("Hello Pico")])
        .unwrap()
        .build();

    loop {
        // Poll services USB enumeration and transfer events. The serial write
        // is attempted whenever the USB stack reports an event.
        if usb_device.poll(&mut [&mut serial]) {
            // `write` can return WouldBlock when the USB endpoint is busy. The
            // demo intentionally ignores that result to stay minimal.
            let _ = serial.write(b"Hello from Pico!\r\n");
        }
    }
}
