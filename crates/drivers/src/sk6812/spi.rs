use std::{
    io::{self, Write},
    time::Duration,
};

use spidev::{SpiModeFlags, Spidev, SpidevOptions};

// Hardcoded so a stale WS2801 freq_hz / latch_time_micros can't break the waveform.
pub const SPI_FREQ_HZ: u32 = 3_200_000;
pub const RESET: Duration = Duration::from_micros(80);

// spidev rejects writes larger than its `bufsiz` (default 4096).
// Chunking lets long strips work without needing spidev.bufsiz= on cmdline.
const MAX_CHUNK: usize = 4096;

pub fn open(spi_path: Option<&str>, kind: &str) -> io::Result<Spidev> {
    let path = spi_path.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("spi_path not specified for {kind}"),
        )
    })?;
    let mut device = Spidev::open(path)?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(SPI_FREQ_HZ)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    device.configure(&options)?;
    Ok(device)
}

pub fn write_frame(device: &mut Spidev, buf: &[u8]) -> io::Result<()> {
    for chunk in buf.chunks(MAX_CHUNK) {
        device.write_all(chunk)?;
    }
    Ok(())
}
