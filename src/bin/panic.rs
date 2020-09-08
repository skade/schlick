#![no_main]
#![no_std]

use schlick as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("main");

    panic!()
}
