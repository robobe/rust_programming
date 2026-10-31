#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::{entry, hal, pac};
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Rust course")
            .product("Hello Pico")
            .serial_number("PICO")])
        .unwrap()
        .device_class(2)
        .build();

    loop {
        if usb_device.poll(&mut [&mut serial]) {
            let _ = serial.write(b"Hello, Pico!\r\n");
        }
    }
}
