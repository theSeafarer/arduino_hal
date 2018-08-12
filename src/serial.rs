use hal::serial;
use nb;

use ux::{u5, u6, u7};
use core::prelude::v1::*;
use core::ptr::{read_volatile, write_volatile};
use core::marker::PhantomData;
use super::reg::*;


#[inline(always)]
pub fn ubrr_with(frq_hz: u64, baud_rate: u64) -> u16 {
    return (frq_hz / 16 / baud_rate - 1) as u16;
}


pub enum SerialError {
    NotReadyToTransmit
}

pub enum Mode {
    Asynchronous,
    Synchronous,
    MasterSpi,
}

impl Mode {
    #[inline]
    fn bits(&self) -> u8 {
        use self::Mode::*;

        match *self {
            Asynchronous => 0       | 0,
            Synchronous  => 0       | UMSEL00,
            // Reserved  => UMSEL01 | 0,
            MasterSpi    => UMSEL01 | UMSEL00,
        }
    }

    #[inline]
    fn mask() -> u8 {
        !(UMSEL01 | UMSEL00)
    }
}

pub enum Parity {
    Disabled,
    Even,
    Odd,
}

impl Parity {
    #[inline]
    fn bits(&self) -> u8 {
        use self::Parity::*;

        match *self {
            Disabled    => 0     | 0,
            // Reserved => 0     | UPM00,
            Even        => UPM01 | 0,
            Odd         => UPM01 | UPM00,
        }
    }

    #[inline]
    fn mask() -> u8 {
        !(UPM01 | UPM00)
    }
}

pub enum StopBits {
    OneBit,
    TwoBits,
}

impl StopBits {
    #[inline]
    fn bits(&self) -> u8 {
        use self::StopBits::*;

        match *self {
            OneBit  => 0,
            TwoBits => USBS0,
        }
    }

    #[inline]
    fn mask() -> u8 {
        !USBS0
    }
}

struct Serial<T> {
  // ubrr: u16,
  // parity: Parity,
  // stop_bits: StopBits
  // mode: Mode
  phantom: PhantomData<T> //pathetic!
}


trait CharSizeFlag {
  fn bits() -> (u8, u8);
}

impl CharSizeFlag for u8 {
    #[inline]
    fn bits() -> (u8, u8) {
        (0, UCSZ01 | UCSZ00)
    }
}

impl CharSizeFlag for u5 {
    #[inline]
    fn bits() -> (u8, u8) {
        (0, 0)
    }
}

impl CharSizeFlag for u6 {
    #[inline]
    fn bits() -> (u8, u8) {
        (0, 0 | UCSZ00)
    }
}

impl CharSizeFlag for u7 {
    #[inline]
    fn bits() -> (u8, u8) {
        (0, UCSZ01 | 0)
    }
}

//no u9, because I have no idea what to do about it

impl<T> Serial<T> where
  T: CharSizeFlag + Into<u8> {
  #[inline]
  pub fn new(ubrr: u16, parity: Parity, stop: StopBits) -> Self {
    let mut b: u8 = 0;
    let mut c: u8 = 0;
    
    //character size config
    let (cb, cc) = T::bits();
    b |= cb;
    c |= cc;
    b &= !(UCSZ01 | UCSZ00);
    c &= !(UCSZ02);

    //initial mode config
    let mode = Mode::Asynchronous;
    c &= Mode::mask();
    c |= mode.bits();

    //parity config
    c &= Parity::mask();
    c |= parity.bits();

    //stop bits config
    c &= StopBits::mask();
    c |= stop.bits();

    //writing the configs
    unsafe {
        write_volatile(UBRR0, ubrr);
        write_volatile(UCSR0A, 0);
        write_volatile(UCSR0B, b | RXEN0 | TXEN0);
        write_volatile(UCSR0C, c);
    }

    Serial {
        phantom: PhantomData
    }
  }

}

#[inline]
pub fn ready_to_transmit() -> bool {
    unsafe { (read_volatile(UCSR0A) & UDRE0) != 0 }
}

#[inline]
fn do_write<T>(word: T) where
  T: Into<u8> {
    unsafe { write_volatile(UDR0, word.into()); }
}

impl<T> serial::Write<T> for Serial<T> where
  T: Into<u8> {

    type Error = SerialError;

    #[inline]
    fn write(&mut self, word: T) -> nb::Result<(), Self::Error> {
        use self::SerialError::*;
        if ready_to_transmit() {
            do_write(word);
            Ok(())
        }
        else {
            Err(nb::Error::Other(NotReadyToTransmit))
        }
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }

}

