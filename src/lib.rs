#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(never_type)]
#![no_std]
#![feature(lang_items)]

extern crate embedded_hal as hal;
extern crate nb;
extern crate ux;

pub mod serial;
pub mod spi;
pub mod timer;
pub mod reg;