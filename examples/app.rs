//! Example application
//!
//! This shows all the parts that are required to build an application

// We don't link to the `std` crate because it's not available for Cortex-M
// devices.
#![no_std]

// We have to link our crate, obviously
extern crate demopill;

// Instead of `std` we use the `core` crate, which provides the subset of
// `std`'s functionality that works on bare metal environments
use core::u32;

use demopill::exceptions::{self, Exceptions};
use demopill::interrupts::{self, Interrupts};

// We need a `main` function, just like every other Rust program
fn main() {
    loop {
        let y: u32;
        let mut x: u32 = 0xDEADBEEF;
        bar(&mut x);
        y = u32::MAX;
    }
}

fn bar(z: &mut u32) {
    *z += 1;
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
