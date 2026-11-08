# Module 0: Hello, Rust and tooling

By the end, you will be able to install Rust and build, run, format, and check
a Hello World application.

- Install and understand the Rust toolchain.
- Create and run a Cargo application.
- Explore optional packaging, compiler-output, workspace, and embedded workflows.

Prerequisites: a terminal and basic programming familiarity. All five topics
are published. Complete Topic 1 for the main course; Topics 2–5 are optional
extensions and are not prerequisites for Module 1.

---

## 1. Set up Rust and build Hello World

- Install stable Rust with `rustup`.
- Understand `rustc`, Cargo, formatting, linting, and editor tools.
- Create, build, run, check, test, and package a small application.

Start with [Setup and Hello World](01-setup-and-hello-world.md).

---

## 2. From source to DEB — optional

- Build an optimized release binary.
- Package an application as a Debian `.deb` file.
- Install and remove the package on Debian or Ubuntu.

After Topic 1, explore [From source to DEB](02-from-source-to-deb.md).

---

## 3. Linux compiler output — optional

- Locate Cargo's generated files on Linux.
- Compare debug and release output.
- Inspect executable and library artifacts.

After Topic 1, explore [Linux compiler output](03-linux-compiler-output.md).

---

## 4. Cargo workspace with a binary and library — optional

- Group related packages under a workspace manifest.
- Separate an executable from reusable library code.
- Organize library modules and call them from the executable.

After Topic 1, explore [Cargo workspaces](04-cargo-workspace.md).

---

## 5. Cross-compile for Raspberry Pi Pico — optional

- Build firmware for the RP2040 target.
- Flash through the USB bootloader and print over USB serial.
- Understand the role of a debug probe.

After Topic 1, explore [Raspberry Pi Pico cross-compilation](05-rp2040-cross-compile.md).
Follow that lesson's hardware and tool prerequisites.

---

## Completion and next module

You are ready to continue when you can run Hello World and explain which
commands build, check, and format it.

Next: [Module 1: Rust fundamentals](../01-fundamentals/index.md).
