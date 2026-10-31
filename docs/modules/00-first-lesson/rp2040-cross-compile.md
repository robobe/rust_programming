# Cross-compile Rust for a Raspberry Pi Pico

## By the end, you will be able to

- Build Rust firmware for the RP2040 Cortex-M0+ processor.
- Send `Hello, Pico!` to a computer over USB serial.
- Flash the firmware through the Pico USB bootloader.
- Understand how a debug probe enables breakpoints and stepping.

---

## What changes on a Pico?

A Linux Rust program targets the host operating system. Pico firmware is
bare-metal code, so it needs a different target, a board memory layout, a
startup runtime, and a flashing method.

The RP2040 uses two Cortex-M0+ cores. Rust targets it with
`thumbv6m-none-eabi`.

```mermaid
flowchart LR
  A[Rust source] --> B[rustc for thumbv6m-none-eabi]
  B --> C[ARM ELF firmware]
  C --> D[UF2 or SWD flash]
  D --> E[RP2040 Pico]
  E --> F[USB CDC serial output]
```

The `rp-pico` board-support package configures the Pico pins and RP2040 HAL.
The `usb-device` and `usbd-serial` crates expose a USB CDC-ACM serial port.

## Install the embedded toolchain

```bash
rustup target add thumbv6m-none-eabi
cargo install --locked elf2uf2-rs
```

`elf2uf2-rs` converts the ARM ELF output to UF2 and can copy it to the mounted
`RPI-RP2` bootloader drive. The demo uses `rust-lld`, so a separate ARM GCC
compiler is not required.

## Project configuration

The demo's `.cargo/config.toml` selects the cross target and linker script:

```toml
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
linker = "rust-lld"
runner = "elf2uf2-rs -d"
rustflags = [
    "-C", "link-arg=--nmagic",
    "-C", "link-arg=-Tlink.x",
]
```

`memory.x` describes the Pico's flash and RAM layout. `build.rs` copies it to
the linker search path so `cortex-m-rt` can use it reliably.

## USB Hello Pico application

The application is `#![no_std]` and `#![no_main]`. It initializes the RP2040
clocks, creates a USB CDC serial class, and writes a greeting while polling USB.

```rust
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
```

The complete compilable version is in the demo directory.

## Build the firmware

From the repository root, enter the demo directory first so Cargo discovers
the embedded `.cargo/config.toml`:

```bash
cd examples/00-first-lesson/rp2040-hello-pico
cargo check
cargo build --release
```

The final ELF is written under:

```text
target/thumbv6m-none-eabi/release/rp2040-hello-pico
```

## Flash over USB bootloader

1. Hold the Pico's `BOOTSEL` button.
2. Connect the Pico to USB.
3. Release `BOOTSEL` after the `RPI-RP2` drive appears.
4. Run:

```bash
cargo run --release
```

The configured `elf2uf2-rs -d` runner converts and copies the firmware. You can
also create a UF2 file without flashing it:

```bash
elf2uf2-rs target/thumbv6m-none-eabi/release/rp2040-hello-pico hello-pico.uf2
```

Copy `hello-pico.uf2` to the mounted `RPI-RP2` drive, then wait for the Pico to
reboot.

## Read the USB output

After the firmware boots, a new USB serial device should appear. On Linux it is
often `/dev/ttyACM0`:

```bash
ls /dev/ttyACM*
screen /dev/ttyACM0 115200
```

The USB CDC connection does not use a physical baud rate, so `115200` is only a
conventional terminal setting. Press `Ctrl-A`, then `K` to exit `screen`.

## Debugging the Pico

Yes, but USB serial output is logging, not source-level debugging. For
breakpoints, stepping, register inspection, and memory inspection, use SWD
with a debug probe such as a Raspberry Pi Debug Probe, J-Link, or a second Pico
flashed with CMSIS-DAP debug-probe firmware.

Connect `SWCLK`, `SWDIO`, and `GND` from the probe to the target Pico. Install
probe-rs and list the probe:

```bash
cargo install --locked probe-rs-tools
probe-rs list
```

Temporarily change the runner to:

```toml
runner = "probe-rs run --chip RP2040"
```

Then flash and run through SWD:

```bash
cargo run
```

The probe-rs VS Code extension can provide breakpoints and stepping. Keep
debug information enabled in the debug profile so source lines map correctly.

---

## Checkpoint

<div class="quiz" data-answer="b">
<p>Which Rust target is used for an RP2040 Pico?</p>
<label><input type="radio" name="pico-target" value="a"> <code>x86_64-unknown-linux-gnu</code></label><br>
<label><input type="radio" name="pico-target" value="b"> <code>thumbv6m-none-eabi</code></label><br>
<label><input type="radio" name="pico-target" value="c"> <code>wasm32-unknown-unknown</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result"></p>
</div>
