# Overview

This repository provides examples of using Rust to program the [GD32VF103 MCU].
All of these build on foundational crates provided by the
[Rust Embedded Working Group].

Available examples:

* [*blinky*](./src/blinky.rs)
* [*hello*](./src/hello.rs)
* [*echo*](./src/echo.rs)

[GD32VF103 MCU]: https://www.gigadevice.com/products/microcontrollers/gd32/risc-v/
[Rust Embedded Working Group]: https://github.com/rust-embedded/wg

# Building

These examples can be compiled and flashed from any Windows or Linux system.

Prerequisites:

* Stable Rust w/ `riscv32imac-unknown-none-elf` target
* [Nuclei RISC-V toolchain] (needed only for objcopy)
* [Modified dfu-util] with support for GD32V bootloader
  * As mentioned in the release notes, Windows users must also use [Zadig] to
    configure WinUSB for the enumerated USB bootloader
* Clone this repository and initialize submodules

To build and flash *blinky*:

```
cargo build --bin blinky --release
riscv-nuclei-elf-objcopy target/riscv32imac-unknown-none-elf/release/blinky -O binary -S blinky.bin
dfu-util -d 28e9:0189 -a 0 --dfuse-address 0x08000000:leave -D blinky.bin
```

Note that *dfu-util* requires the device to be in bootloader mode.

[Nuclei RISC-V toolchain]: https://nucleisys.com/download.php
[Modified dfu-util]: https://github.com/riscv-mcu/gd32-dfu-utils
[Zadig]: https://zadig.akeo.ie/

# License

Licensed under either of:

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
