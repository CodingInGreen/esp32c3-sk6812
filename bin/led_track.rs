#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::{GpioPin, Output, IO}, ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource,
        LowSpeed,
        LEDC,
    }, peripherals::Peripherals, prelude::*, Delay
};

#[entry]
fn main() -> ! {
    todo!();
}