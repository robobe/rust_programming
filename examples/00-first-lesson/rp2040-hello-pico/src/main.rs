#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::{entry, hal, pac};
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);
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

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        peripherals.USBCTRL_REGS,
        peripherals.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut peripherals.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default().product("Hello Pico")])
        .unwrap()
        .build();

    loop {
        if usb_device.poll(&mut [&mut serial]) {
            let _ = serial.write(b"Hello from Pico!\r\n");
        }
    }
}
