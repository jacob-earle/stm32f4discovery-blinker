#![no_std]
#![no_main]

// Based on inspiration here
// https://jonathanklimt.de/electrics/programming/rust-STM32F103-blink/

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    hprintln!("Initializing GPIO!").unwrap();
    let peripherals = stm32f407::Peripherals::take().unwrap();
    let gpiod = &peripherals.GPIOD;
    let rcc = &peripherals.RCC;


    // initialize gpio clock
    rcc.ahb1enr.write(|w| w.gpioden().bit(true));
    // initialize gpio mode to output
    gpiod.moder.write(|w| w.moder13().bits(0b01));
    // initialize output speed to high
    gpiod.ospeedr.write(|w| w.ospeedr13().bits(0b10));

    hprintln!("Ready to flip light!").unwrap();
    loop {
        // constantly flip lights
        gpiod.bsrr.write(|w| w.bs13().set_bit());
        cortex_m::asm::delay(16000000);
        gpiod.bsrr.write(|w| w.br13().set_bit());
        cortex_m::asm::delay(16000000);

    }
}
