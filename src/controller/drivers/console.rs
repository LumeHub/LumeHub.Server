use crate::{
    controller::{Controller, color::Rgb},
    impl_pixel_access_for_controller,
};
use std::io::{self, Write};

pub struct Console {
    pixels: Vec<Rgb>,
}

impl Console {
    pub fn new(len: usize) -> Self {
        Self {
            pixels: vec![Rgb::BLACK; len],
        }
    }
}

impl Controller for Console {
    fn show(&mut self) {
        // print the pixels
        self.as_ref().iter().for_each(|color| {
            print!("\x1b[48;2;{};{};{}m \x1b[0m", color.r, color.g, color.b);
        });
        println!(); // next line
        print!("\x1b[0m"); // reset color
        io::stdout().flush().unwrap(); // force terminal to display;
    }
}

impl_pixel_access_for_controller!(Console);
