pub mod color;
pub mod drivers;

#[allow(dead_code)]
pub trait Controller {
    fn len(&self) -> usize;
    fn show(&mut self);
    fn fill(&mut self, color: color::Rgb) {
        self.map(Box::new(move |_, _| color));
    }
    fn map(&mut self, f: Box<dyn FnMut(usize, color::Rgb) -> color::Rgb>);
}

pub fn create_controller(
    config: &crate::settings::LedControllerConfig,
) -> Result<Box<dyn Controller>, std::io::Error> {
    match config.controller_type {
        crate::settings::ControllerType::Console => {
            Ok(Box::new(drivers::console::Console::new(config.pixel_count)))
        }
        crate::settings::ControllerType::Ws2801 => {
            Ok(Box::new(drivers::ws2801::Ws2801::new(config)?))
        }
    }
}
