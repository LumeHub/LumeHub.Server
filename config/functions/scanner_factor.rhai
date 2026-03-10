// Gaussian scanner dot: returns a brightness factor 0–1.
// width: Gaussian sigma in pixels (larger = wider/softer dot)
fn scanner_factor(pixel, len, time, speed, width) {
    let period = 2 * (len - 1);
    let phase  = (time * speed).to_int() % period;
    let head   = if phase < len { phase } else { period - phase };
    let dist   = abs(pixel.to_float() - head.to_float());
    exp(-dist * dist / (2.0 * width * width))
}
