use domain::Rgb;

// 2 LED bits per SPI byte at 3.2 MHz: "0" -> 0b1000, "1" -> 0b1100.
// Timings land inside SK6812 T0H/T0L/T1H/T1L windows.
const PATTERN_LUT: [u8; 4] = [
    0b1000_1000, // 00
    0b1000_1100, // 01
    0b1100_1000, // 10
    0b1100_1100, // 11
];

pub fn encode_grb(pixels: &[Rgb]) -> Vec<u8> {
    let mut out = Vec::with_capacity(pixels.len() * 12);
    for p in pixels {
        let bits: u32 = ((p.g as u32) << 16) | ((p.r as u32) << 8) | (p.b as u32);
        for i in (0..12).rev() {
            let pair = ((bits >> (i * 2)) & 0b11) as usize;
            out.push(PATTERN_LUT[pair]);
        }
    }
    out
}

// W = min(r,g,b) subtracted from each channel so the dedicated white LED
// carries the achromatic floor and RGB only carries color.
pub fn encode_grbw(pixels: &[Rgb]) -> Vec<u8> {
    let mut out = Vec::with_capacity(pixels.len() * 16);
    for p in pixels {
        let w = p.r.min(p.g).min(p.b);
        let r = p.r - w;
        let g = p.g - w;
        let b = p.b - w;
        let bits: u32 = ((g as u32) << 24) | ((r as u32) << 16) | ((b as u32) << 8) | (w as u32);
        for i in (0..16).rev() {
            let pair = ((bits >> (i * 2)) & 0b11) as usize;
            out.push(PATTERN_LUT[pair]);
        }
    }
    out
}
