use crate::settings::LedControllerConfig;
use crate::{
    controller::{Controller, color::Rgb},
    impl_pixel_access_for_controller,
};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

pub struct Ws2801 {
    device: Spidev,
    pixels: Vec<Rgb>,
    latch_time: Duration,
}

impl Ws2801 {
    pub fn new(config: &LedControllerConfig) -> std::io::Result<Self> {
        let spi_path = config.spi_path.as_deref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "spi_path not specified for ws2801 controller",
            )
        })?;
        let mut device = Spidev::open(spi_path)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(config.freq_hz.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "freq_hz not specified for ws2801 controller",
                )
            })?)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        device.configure(&options)?;

        Ok(Self {
            device,
            pixels: vec![Rgb::BLACK; config.pixel_count],
            latch_time: Duration::from_micros(config.latch_time_micros.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "latch_time_micros not specified for ws2801 controller",
                )
            })?),
        })
    }
}

impl Controller for Ws2801 {
    fn show(&mut self) {
        let buf: Vec<u8> = self
            .as_ref()
            .iter()
            .flat_map(|c| vec![c.r, c.g, c.b])
            .collect();
        let _ = self.device.write_all(&buf);
        thread::sleep(self.latch_time);
    }
}

impl_pixel_access_for_controller!(Ws2801);
