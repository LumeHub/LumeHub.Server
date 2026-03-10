// Single meteor: returns a brightness factor 0–1.
// offset: phase shift so multiple meteors don't overlap (use multiples of len)
fn meteor_factor(pixel, len, time, speed, tail, offset) {
    let period = 2 * (len - 1);
    let phase  = ((time + offset) * speed).to_int() % period;
    let right  = phase < len;
    let head   = if right { phase } else { period - phase };
    if (right && pixel > head) || (!right && pixel < head) { return 0.0; }
    let behind = if right { head - pixel } else { pixel - head };
    if behind >= tail.to_int() { return 0.0; }
    let f = 1.0 - behind.to_float() / tail;
    f * f
}
