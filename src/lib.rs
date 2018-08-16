// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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