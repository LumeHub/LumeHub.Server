use crate::controller::{Controller, color::Rgb};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{io::Write, thread, time::Duration};

pub struct Ws2801 {
    device: Spidev,
    buf: Vec<u8>,
}

impl Ws2801 {
    pub fn new(spi_path: &str, pixel_count: usize, freq_hz: u32) -> std::io::Result<Self> {
        let mut device = Spidev::open(spi_path)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(freq_hz)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        device.configure(&options)?;

        Ok(Self {
            device,
            buf: vec![0; pixel_count * 3],
        })
    }
}

impl Controller for Ws2801 {
    fn len(&self) -> usize {
        self.buf.len() / 3
    }

    fn map<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, Rgb) -> Rgb,
    {
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
        thread::sleep(Duration::from_micros(500));
    }
}
