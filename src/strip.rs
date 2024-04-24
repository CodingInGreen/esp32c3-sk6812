#![no_std]
#![no_main]

use palette::{Gradient, LinSrgb, Srgb};
use esp_hal::spi::{Spi, FullDuplexMode, SpiMode};
use esp_hal::delay::Delay;
use esp_hal::peripherals::Peripherals;
use esp_hal::peripheral::PeripheralRef;
use esp_hal::gpio::{Output, GpioPin, PullUp, PushPull};
use core::result::Result;
use core::marker::PhantomData;

const SPI_FREQUENCY: u32 = 6_400_000;

#[derive(Debug)]
pub struct Strip {
    spi: Spi<'static, FullDuplexMode>,
    pub leds: Vec<Led>,
}

impl Strip {
    pub fn new(
        spi: PeripheralRef<'static, Spi<FullDuplexMode>>,
        amount_of_leds: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            spi,
            leds: vec![Led::new(); amount_of_leds],
        })
    }

    pub fn fill(&mut self, led: Led) {
        self.leds.fill(led);
    }

    pub fn clear(&mut self) {
        self.leds.fill(Led::new());
    }

    pub fn set_gradient(&mut self, gradient: Gradient<LinSrgb>) {
        gradient
            .take(self.leds.len())
            .zip(&mut self.leds)
            .for_each(|(color, led)| {
                *led = Srgb::from_linear(color).into();
            });
    }

    pub fn shift_left(&mut self, count: usize) {
        self.leds.rotate_left(count);
    }

    pub fn shift_right(&mut self, count: usize) {
        self.leds.rotate_right(count);
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let led_data: Vec<u8> = self.raw_led_data().collect();
        self.spi.write(&led_data)?;
        // Insert delay as required
        Delay::delay_us(80);

        Ok(())
    }

    fn raw_led_data(&self) -> impl Iterator<Item = u8> {
        self.leds.iter().flat_map(|led| led.to_raw_led_bytes())
    }
}

impl core::ops::ShrAssign<usize> for Strip {
    fn shr_assign(&mut self, rhs: usize) {
        self.shift_right(rhs);
    }
}

impl core::ops::ShlAssign<usize> for Strip {
    fn shl_assign(&mut self, rhs: usize) {
        self.shift_left(rhs);
    }
}

