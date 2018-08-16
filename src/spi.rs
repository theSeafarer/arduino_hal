// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hal::spi;
use hal::blocking::spi as bspi;

use core::ptr::{read_volatile, write_volatile};
use super::reg::*;

pub enum MasterState {
  Read,
  Sent
}

