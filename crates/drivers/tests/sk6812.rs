use domain::Rgb;
use drivers::encode_grb;

const BYTES_FOR_ZERO_CHANNEL: [u8; 4] = [0x88, 0x88, 0x88, 0x88];
const BYTES_FOR_FULL_CHANNEL: [u8; 4] = [0xCC, 0xCC, 0xCC, 0xCC];

#[test]
fn empty_input_yields_empty_buffer() {
    assert!(encode_grb(&[]).is_empty());
}

#[test]
fn twelve_bytes_per_pixel() {
    let buf = encode_grb(&[Rgb::BLACK; 3]);
    assert_eq!(buf.len(), 36);
}

#[test]
fn black_encodes_to_zero_pattern_three_channels() {
    let buf = encode_grb(&[Rgb::BLACK]);
    let mut expected = Vec::new();
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    assert_eq!(buf, expected);
}

#[test]
fn white_encodes_to_one_pattern_three_channels() {
    let buf = encode_grb(&[Rgb::new(255, 255, 255)]);
    let mut expected = Vec::new();
    expected.extend_from_slice(&BYTES_FOR_FULL_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_FULL_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_FULL_CHANNEL);
    assert_eq!(buf, expected);
}

#[test]
fn pure_green_takes_first_four_bytes_grb_order() {
    let buf = encode_grb(&[Rgb::new(0, 255, 0)]);
    let mut expected = Vec::new();
    expected.extend_from_slice(&BYTES_FOR_FULL_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    assert_eq!(buf, expected);
}

#[test]
fn pure_red_takes_middle_four_bytes_grb_order() {
    let buf = encode_grb(&[Rgb::new(255, 0, 0)]);
    let mut expected = Vec::new();
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_FULL_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    assert_eq!(buf, expected);
}

#[test]
fn pure_blue_takes_last_four_bytes_grb_order() {
    let buf = encode_grb(&[Rgb::new(0, 0, 255)]);
    let mut expected = Vec::new();
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_ZERO_CHANNEL);
    expected.extend_from_slice(&BYTES_FOR_FULL_CHANNEL);
    assert_eq!(buf, expected);
}

#[test]
fn multiple_pixels_concatenate_with_no_gap() {
    let buf = encode_grb(&[Rgb::BLACK, Rgb::new(255, 255, 255)]);
    assert_eq!(buf.len(), 24);
    assert_eq!(&buf[0..12], &encode_grb(&[Rgb::BLACK])[..]);
    assert_eq!(&buf[12..24], &encode_grb(&[Rgb::new(255, 255, 255)])[..]);
}
