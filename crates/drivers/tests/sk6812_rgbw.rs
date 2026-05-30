use domain::Rgb;
use drivers::encode_grbw;

const BYTES_FOR_ZERO_CHANNEL: [u8; 4] = [0x88, 0x88, 0x88, 0x88];
const BYTES_FOR_FULL_CHANNEL: [u8; 4] = [0xCC, 0xCC, 0xCC, 0xCC];

fn expected(g: &[u8; 4], r: &[u8; 4], b: &[u8; 4], w: &[u8; 4]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(g);
    out.extend_from_slice(r);
    out.extend_from_slice(b);
    out.extend_from_slice(w);
    out
}

#[test]
fn empty_input_yields_empty_buffer() {
    assert!(encode_grbw(&[]).is_empty());
}

#[test]
fn sixteen_bytes_per_pixel() {
    let buf = encode_grbw(&[Rgb::BLACK; 3]);
    assert_eq!(buf.len(), 48);
}

#[test]
fn black_encodes_to_all_zero_channels() {
    let buf = encode_grbw(&[Rgb::BLACK]);
    let expected = expected(
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
    );
    assert_eq!(buf, expected);
}

#[test]
fn pure_white_extracts_into_w_and_zeros_rgb() {
    let buf = encode_grbw(&[Rgb::new(255, 255, 255)]);
    let expected = expected(
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_FULL_CHANNEL,
    );
    assert_eq!(buf, expected);
}

#[test]
fn pure_red_passes_through_with_zero_w() {
    let buf = encode_grbw(&[Rgb::new(255, 0, 0)]);
    let expected = expected(
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_FULL_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
    );
    assert_eq!(buf, expected);
}

#[test]
fn pure_green_passes_through_with_zero_w() {
    let buf = encode_grbw(&[Rgb::new(0, 255, 0)]);
    let expected = expected(
        &BYTES_FOR_FULL_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
    );
    assert_eq!(buf, expected);
}

#[test]
fn pure_blue_passes_through_with_zero_w() {
    let buf = encode_grbw(&[Rgb::new(0, 0, 255)]);
    let expected = expected(
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
        &BYTES_FOR_FULL_CHANNEL,
        &BYTES_FOR_ZERO_CHANNEL,
    );
    assert_eq!(buf, expected);
}

#[test]
fn neutral_gray_extracts_fully_into_w() {
    let buf = encode_grbw(&[Rgb::new(200, 200, 200)]);
    assert_eq!(&buf[0..4], &BYTES_FOR_ZERO_CHANNEL);
    assert_eq!(&buf[4..8], &BYTES_FOR_ZERO_CHANNEL);
    assert_eq!(&buf[8..12], &BYTES_FOR_ZERO_CHANNEL);
    assert_ne!(&buf[12..16], &BYTES_FOR_ZERO_CHANNEL);
}

#[test]
fn tinted_white_splits_into_w_floor_plus_color_tint() {
    let buf = encode_grbw(&[Rgb::new(255, 200, 200)]);
    assert_eq!(&buf[0..4], &BYTES_FOR_ZERO_CHANNEL);
    assert_ne!(&buf[4..8], &BYTES_FOR_ZERO_CHANNEL);
    assert_ne!(&buf[4..8], &BYTES_FOR_FULL_CHANNEL);
    assert_eq!(&buf[8..12], &BYTES_FOR_ZERO_CHANNEL);
    assert_ne!(&buf[12..16], &BYTES_FOR_ZERO_CHANNEL);
}

#[test]
fn multiple_pixels_concatenate_with_no_gap() {
    let buf = encode_grbw(&[Rgb::BLACK, Rgb::new(255, 255, 255)]);
    assert_eq!(buf.len(), 32);
    assert_eq!(&buf[0..16], &encode_grbw(&[Rgb::BLACK])[..]);
    assert_eq!(&buf[16..32], &encode_grbw(&[Rgb::new(255, 255, 255)])[..]);
}
