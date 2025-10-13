use std::io::{self, Write};

use crate::controller::{Controller, color::Rgb};

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
    fn len(&self) -> usize {
        self.pixels.len()
    }

    fn show(&mut self) {
        // print the pixels
        self.pixels.iter().for_each(|color| {
            print!("\x1b[48;2;{};{};{}m \x1b[0m", color.r, color.g, color.b);
        });
        println!(); // next line
        print!("\x1b[0m"); // reset color
        io::stdout().flush().unwrap(); // force terminal to display
    }

    fn map<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, crate::controller::color::Rgb) -> crate::controller::color::Rgb,
    {
        for (i, p) in self.pixels.iter_mut().enumerate() {
            *p = f(i, *p);
        }
    }
}
