pub mod color;
pub mod drivers;

use serde::{Deserialize, Serialize};

pub trait Controller {
    fn len(&self) -> usize;
    fn show(&mut self);
    fn fill(&mut self, color: color::Rgb) {
        self.map(Box::new(move |_, _| color.clone()));
    }
    fn map(&mut self, f: Box<dyn FnMut(usize, color::Rgb) -> color::Rgb>);
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum ControllerType {
    Console,
    Ws2801,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LedControllerConfig {
    pub controller_type: ControllerType,
    pub pixel_count: usize,
    pub spi_path: Option<String>,
    pub freq_hz: Option<u32>,
}

pub fn create_controller(config: &LedControllerConfig) -> Result<Box<dyn Controller>, std::io::Error> {
    match config.controller_type {
        ControllerType::Console => Ok(Box::new(drivers::console::Console::new(config.pixel_count))),
        ControllerType::Ws2801 => {
            let spi_path = config.spi_path.as_deref().unwrap_or("/dev/spidev0.0");
            let freq_hz = config.freq_hz.unwrap_or(1_000_000);
            Ok(Box::new(drivers::ws2801::Ws2801::new(spi_path, config.pixel_count, freq_hz).map_err(|e| std::io::Error::new(e.kind(), format!("Failed to create Ws2801 controller at '{}': {}", spi_path, e)))?))
        }
    }
}
