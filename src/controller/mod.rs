pub mod drivers;
pub mod effect_processor;

use crate::color::Rgb;
use crate::settings::{ControllerType, LedControllerConfig};
use drivers::{console::Console, ws2801::Ws2801};

#[macro_export]
macro_rules! impl_pixel_access_for_controller {
    ($struct_name:ident) => {
        impl AsRef<[Rgb]> for $struct_name {
            fn as_ref(&self) -> &[Rgb] {
                &self.pixels
            }
        }

        impl AsMut<[Rgb]> for $struct_name {
            fn as_mut(&mut self) -> &mut [Rgb] {
                &mut self.pixels
            }
        }
    };
}

pub trait Controller: Send + AsRef<[Rgb]> + AsMut<[Rgb]> {
    fn as_pixel_slice(&self) -> &[Rgb] {
        self.as_ref()
    }
    fn as_pixel_slice_mut(&mut self) -> &mut [Rgb] {
        self.as_mut()
    }
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

pub fn create_controller(
    config: &LedControllerConfig,
) -> Result<Box<dyn Controller>, std::io::Error> {
    match config.controller_type {
        ControllerType::Console => Ok(Box::new(Console::new(config.pixel_count))),
        ControllerType::Ws2801 => Ok(Box::new(Ws2801::new(config)?)),
    }
}
