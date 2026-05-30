use std::thread;

use domain::Rgb;
use spidev::Spidev;

use crate::{Controller, DriverConfig, impl_pixel_access};

use super::encoding::encode_grb;
use super::spi;

pub struct Sk6812 {
    device: Spidev,
    pixels: Vec<Rgb>,
}

impl Sk6812 {
    pub fn new(config: &DriverConfig) -> std::io::Result<Self> {
        Ok(Self {
            device: spi::open(config.spi_path.as_deref(), "sk6812")?,
            pixels: vec![Rgb::BLACK; config.pixel_count],
        })
    }
}

impl Controller for Sk6812 {
    fn show(&mut self) {
        let buf = encode_grb(self.as_ref());
        if let Err(e) = spi::write_frame(&mut self.device, &buf) {
            eprintln!("warning: sk6812 SPI write failed: {e}");
        }
        thread::sleep(spi::RESET);
    }
}

impl_pixel_access!(Sk6812);
