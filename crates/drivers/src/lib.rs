use domain::Rgb;
use serde::{Deserialize, Serialize};

mod console;
mod ws2801;

pub use console::Console;
pub use ws2801::Ws2801;

macro_rules! impl_pixel_access {
    ($t:ident) => {
        impl AsRef<[Rgb]> for $t {
            fn as_ref(&self) -> &[Rgb] {
                &self.pixels
            }
        }
        impl AsMut<[Rgb]> for $t {
            fn as_mut(&mut self) -> &mut [Rgb] {
                &mut self.pixels
            }
        }
    };
}
use impl_pixel_access;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ControllerType {
    Console,
    Ws2801,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DriverConfig {
    pub controller_type: ControllerType,
    pub pixel_count: usize,
    pub spi_path: Option<String>,
    pub freq_hz: Option<u32>,
    pub latch_time_micros: Option<u64>,
}

pub trait Controller: Send + AsRef<[Rgb]> + AsMut<[Rgb]> {
    fn show(&mut self);
}

impl AsRef<[Rgb]> for Box<dyn Controller> {
    fn as_ref(&self) -> &[Rgb] {
        (**self).as_ref()
    }
}

impl AsMut<[Rgb]> for Box<dyn Controller> {
    fn as_mut(&mut self) -> &mut [Rgb] {
        (**self).as_mut()
    }
}

impl engine::Output for Box<dyn Controller> {
    fn pixels(&self) -> &[Rgb] {
        self.as_ref()
    }
    fn pixels_mut(&mut self) -> &mut [Rgb] {
        self.as_mut()
    }
    fn show(&mut self) {
        (**self).show()
    }
}

pub fn create(config: &DriverConfig) -> Result<Box<dyn Controller>, std::io::Error> {
    match config.controller_type {
        ControllerType::Console => Ok(Box::new(Console::new(config.pixel_count))),
        ControllerType::Ws2801 => Ok(Box::new(Ws2801::new(config)?)),
    }
}
