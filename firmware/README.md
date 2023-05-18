# T-monitor Firmware
This firmware runs on a ESP32.

## Install
Not being officially supported by the Rust compiler, we should first install the ESP Espressifi's llvm fork. You can check the instructions [here](https://github.com/esp-rs/rust).

To build just run `cargo build`.

## Flash and monitor
There are a couple of tools useful for development:
- [espmonitor](https://github.com/esp-rs/espmonitor)
- [espflash](https://github.com/esp-rs/espflash)

Once installed, you can flash the microcontroller using:
```
cargo espflash
```

or, you can monitor and read from its serial using:
```
cargo espmonitor <tty>
```

or, flash and monitor:
```
cargo espmonitor --flash <tty>
```
