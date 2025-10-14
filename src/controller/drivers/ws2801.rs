use crate::controller::{Controller, color::Rgb};
use crate::settings::LedControllerConfig;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{io::Write, thread, time::Duration};

pub struct Ws2801 {
    device: Spidev,
    buf: Vec<u8>,
    latch_time: Duration,
}

impl Ws2801 {
    pub fn new(config: &LedControllerConfig) -> std::io::Result<Self> {
        let spi_path = config.spi_path.as_deref().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "spi_path not specified for ws2801 controller"))?;
        let mut device = Spidev::open(spi_path)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(config.freq_hz.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "freq_hz not specified for ws2801 controller"))?)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        device.configure(&options)?;

        Ok(Self {
            device,
            buf: vec![0; config.pixel_count * 3],
            latch_time: Duration::from_micros(config.latch_time_micros.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "latch_time_micros not specified for ws2801 controller"))?),
        })
    }
}

impl Controller for Ws2801 {
    fn len(&self) -> usize {
        self.buf.len() / 3
    }

    fn map(&mut self, mut f: Box<dyn FnMut(usize, Rgb) -> Rgb>) {
        self.buf
            .chunks_exact_mut(3)
            .enumerate()
            .for_each(|(i, chunk)| {
                let old = Rgb {
                    r: chunk[0],
                    g: chunk[1],
                    b: chunk[2],
                };
                let new = f(i, old);
                chunk[0] = new.r;
                chunk[1] = new.g;
                chunk[2] = new.b;
            });
    }

    fn show(&mut self) {
        let _ = self.device.write_all(&self.buf);
        thread::sleep(self.latch_time);
    }
}
