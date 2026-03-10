let x  = pixel.to_float() / len.to_float();
let t  = time * speed * 0.011;
let w1 = sin(x * 3.14  + t)        * 0.5 + 0.5;
let w2 = sin(x * 5.50  - t * 1.6)  * 0.5 + 0.5;
let w3 = sin(x * 1.80  + t * 0.55) * 0.5 + 0.5;
let intensity = pow(w1 * w2 * w3, 0.55);
let hue = 120.0 + sin(t * 0.4 + x * 2.5) * 38.0;
dim(from_hue(hue), intensity)
