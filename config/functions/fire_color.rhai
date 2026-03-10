// Hash-based fire. pixel 0 is hottest; heat decays toward tip.
// flicker: frames per slot (lower = faster flicker)
// turbulence: noise amplitude (0.0–1.0)
// decay: heat falloff exponent (higher = shorter flame)
fn fire_color(pixel, len, frame, flicker, turbulence, decay) {
    let slot  = frame / flicker.to_int();
    let frac  = (frame % flicker.to_int()).to_float() / flicker;
    let n1    = hash((pixel * 7) ^ (slot * 97)) - 0.5;
    let n2    = hash((pixel * 7) ^ ((slot + 1) * 97)) - 0.5;
    let noise = mix(n1, n2, frac) * turbulence;
    let base  = pow(1.0 - pixel.to_float() / len.to_float(), decay);
    heat_color(clamp(base + noise, 0.0, 1.0))
}
