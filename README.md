# nucleo-l432

> Rust on STM32L432 Nucleo-32.

This is a collection of simple programs for the STM32L432 Nucleo-32 board.

## Usage

### Requirements

 - Rust
   - `llvm-tools-preview` component
   - `thumbv7em-none-eabihf` target

 - OpenOCD
 - GDB (`arm-none-eabi-gdb`)

### Build

```
cargo build
```

### Run

Start OpenOCD:

```
openocd -f openocd.cfg
```

Run an debug via GDB:

```
cargo run -p <project>
```

For the build and runner configuration see `.cargo/config`.

## Compatibility

_**Note:**_ I have not tested any other platform.

### STM32L4x2

`memory.x` may need some adjustment.
