# Hello Pico USB serial demo

This is a bare-metal Rust application for the Raspberry Pi Pico with an
RP2040. It prints `Hello from Pico!` through a USB CDC serial port.

The package name is `pico_hellp`, matching the working demo.

## Project files

- `Cargo.toml` — package metadata and dependencies
- `Cargo.lock` — locked dependency versions
- `.cargo/config.toml` — RP2040 target, linker flags, and flash runner
- `src/main.rs` — USB serial firmware
- `memory.x` — RP2040 flash and RAM layout
- `.gitignore` — keeps generated `target/` files out of version control
- `target/` — generated build output, never source code

Install the host tools:

```bash
rustup target add thumbv6m-none-eabi
cargo install --locked elf2uf2-rs
```

Build and check without hardware:

```bash
cd examples/00-first-lesson/rp2040-hello-pico
cargo check
cargo build --release
```

Flash over the USB bootloader by holding `BOOTSEL` while connecting the Pico,
then run:

```bash
cargo run --release
```

After reboot, connect to the USB serial device, commonly `/dev/ttyACM0`:

```bash
screen /dev/ttyACM0 115200
```

The demo was written for the original Raspberry Pi Pico with the RP2040, not
the Pico 2 with the RP2350.
