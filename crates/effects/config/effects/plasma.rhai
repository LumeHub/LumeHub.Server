let x = pixel.to_float() / len.to_float();
let t = time * speed * 0.05;
let v = sin(x * 10.0 + t) + sin(x * 4.0 - t * 0.7)
      + sin(x *  7.0 + t * 0.5) + sin(x * 2.0 + t * 1.3);
from_hue((v + 4.0) / 8.0 * 360.0)
