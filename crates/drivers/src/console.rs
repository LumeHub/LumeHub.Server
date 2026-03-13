use std::io::{self, Write};

use domain::Rgb;

use crate::{Controller, impl_pixel_access};

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
        self.as_ref().iter().for_each(|color| {
            print!("\x1b[48;2;{};{};{}m \x1b[0m", color.r, color.g, color.b);
        });
        println!();
        print!("\x1b[0m");
        io::stdout().flush().unwrap();
    }
}

impl_pixel_access!(Console);
