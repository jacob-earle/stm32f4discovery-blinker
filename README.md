# `stm32f4discovery-blinker`

This project contains example code on how to configure GPIO pins on the STM32F4DISCOVERY board in Rust. In this example, the orange LED labeled LD3 will blink on and off at 1 second intervals.

## Building and Running
Follow the instructions in the [Rust Embedded Book](https://docs.rust-embedded.org/book/intro/install/linux.html) to download the necessary components. Additionally, make sure the `thumbv7em-none-eabi` target is installed on your *nightly* Rust toolchain:

```
rustup target add thumbv7em-none-eabi
```

From here we can compile with 

```
cargo build
```

Now, from the root of this directory, run

```
openocd
```
after plugging in the board using the STLINK-v2 adaptor. In a new terminal, also from the root of this directory run 
```
arm-none-eabi-gdb -q ./target/thumbv7em-none-eabi/debug/stm32f4discovery-blinker
``` 
or on Ubuntu
```
arm-none-eabi-gdb -q ./target/thumbv7em-none-eabi/debug/stm32f4discovery-blinker
```
to open a debugging session. Finally, connect to the board and run the program with
```
(gdb) target remote :3333
...
(gdb) load
...
(gdb) monitor arm semihosting enable
...
(gdb) continue
```

At this point the LED should begin blinking indefinitely.