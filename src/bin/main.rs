#![no_main]
#![no_std]

use defmt::Format;
use rust_no_std_tutorial as _; // global logger + panicking-behavior + memory layout // <- derive attribute

#[derive(Format)]
struct S1<T> {
    x: u8,
    y: T,
}

#[derive(Format)]
struct S2 {
    z: u8,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let s = S1 {
        x: 42,
        y: S2 { z: 43 },
    };
    defmt::println!("s={:?}", s);
    let x = 42;
    defmt::println!("x={=u8}", x);

    rust_no_std_tutorial::exit()
}
