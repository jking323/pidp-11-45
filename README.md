# 🖥️ PiDP-1145

A bare-metal PDP-11/45 emulator running on Raspberry Pi 4 — no OS, just silicon and soul.

![Build](https://github.com/jkin323/pidp-1145/actions/workflows/build.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

---

## 🎯 What is this?

This project emulates a Digital Equipment Corporation PDP-11/45 minicomputer from 1972 on a Raspberry Pi 4, running completely bare-metal — no Linux, no RTOS, just raw ARM64 assembly and Rust talking directly to hardware.

The goal: boot Unix v6 on emulated PDP-11 hardware, with a physical front panel, all running on a $35 computer.

## ✨ Features

- 🔧 **True bare-metal** — Custom bootloader, no OS dependencies
- 🦀 **Written in Rust** — Memory safety meets vintage computing
- 📟 **UART console** — Serial debug output
- 💡 **GPIO control** — Front panel LEDs and switches (planned)
- 🧠 **Full PDP-11/45 emulation** — Including MMU, FP11, and Unibus (in progress)

## 📋 Requirements

### Hardware

- Raspberry Pi 4 (any RAM size)
- MicroSD card (FAT32 formatted)
- USB-UART adapter (or Arduino Pro Micro as bridge)
- Jumper wires for UART connection

### Software

- Rust (nightly or stable with aarch64 target)
- `rust-objcopy` (via `cargo-binutils`)
- Make

## 🚀 Quick Start

### 1. Clone the repo
```bash
git clone https://github.com/YOUR_USERNAME/pidp-1145.git
cd pidp-1145
```

### 2. Install dependencies
```bash
rustup target add aarch64-unknown-none
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### 3. Build
```bash
make release    # Optimized build
make dev        # Debug build with symbols
make clean      # Clean build artifacts
```

### 4. Prepare SD card

Download Raspberry Pi firmware files:
```bash
curl -L -O https://github.com/raspberrypi/firmware/raw/master/boot/start4.elf
curl -L -O https://github.com/raspberrypi/firmware/raw/master/boot/fixup4.dat
```

Create `config.txt`:
```
arm_64bit=1
enable_uart=1
```

Copy to FAT32 SD card:
```
/
├── start4.elf
├── fixup4.dat
├── config.txt
└── kernel8.img    (from build output)
```

### 5. Connect UART
```
USB-UART          Pi 4 GPIO Header
────────          ────────────────
TX       ──────►  Pin 10 (GPIO15/RXD)
RX       ◄──────  Pin 8  (GPIO14/TXD)
GND      ◄──────► Pin 6  (GND)
```

### 6. Boot and connect
```bash
screen /dev/cu.usbmodem* 115200
```

## 🏗️ Project Structure
```
src/
├── main.rs              # Entry point, main loop
├── registers/           # Pi 4 hardware abstraction
│   ├── mod.rs
│   ├── gpio.rs          # GPIO control
│   └── uart.rs          # Serial I/O
└── pdp11/               # PDP-11/45 emulator
    ├── mod.rs
    ├── cpu.rs           # Registers, instruction execution
    ├── psw.rs           # Processor Status Word
    ├── mmu.rs           # Memory Management Unit
    ├── memory.rs        # Physical RAM emulation
    └── devices/         # Unibus peripherals
        ├── console.rs   # DL11 serial terminal
        └── disk.rs      # RK11 disk controller
```

## 🛠️ Build Flags

| Command | Description |
|---------|-------------|
| `make dev` | Debug build, larger binary, symbols preserved |
| `make release` | Optimized build, ~300 bytes for basic kernel |
| `make clean` | Remove all build artifacts |

## 🗺️ Roadmap

- [x] Bare-metal boot on Pi 4
- [x] UART output
- [x] GPIO LED blink
- [ ] UART input (echo)
- [ ] PDP-11 CPU registers
- [ ] Instruction decoder
- [ ] Memory subsystem
- [ ] MMU (memory management)
- [ ] FP11 (floating point)
- [ ] DL11 console
- [ ] RK11 disk
- [ ] Boot Unix v6

## 🤝 Contributing

Contributions welcome! This is a learning project, so:

1. **Fork** the repo
2. **Create** a feature branch (`git checkout -b feature/amazing-thing`)
3. **Commit** your changes (`git commit -m 'Add amazing thing'`)
4. **Push** to the branch (`git push origin feature/amazing-thing`)
5. **Open** a Pull Request

### Code Style

- Run `cargo fmt` before committing
- Keep unsafe blocks minimal and documented
- Comment register magic numbers

## 📚 Resources

- [PDP-11/45 Processor Handbook](http://www.bitsavers.org/pdf/dec/pdp11/handbooks/)
- [BCM2711 ARM Peripherals](https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf)
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest)
- [Unix v6 Source](https://github.com/mit-pdos/xv6-public)

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

*"Those who cannot remember the past are condemned to reimplement it."*
