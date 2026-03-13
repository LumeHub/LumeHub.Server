use domain::Rgb;

#[test]
fn lerp_endpoints_and_midpoint() {
    let a = Rgb::BLACK;
    let b = Rgb::new(100, 100, 100);

    assert_eq!(a.lerp(b, 0.0), a);
    assert_eq!(a.lerp(b, 1.0), b);
    assert_eq!(a.lerp(b, 0.5), Rgb::new(50, 50, 50));
}

#[test]
fn distance_uses_max_channel_difference() {
    let a = Rgb::new(10, 20, 30);
    let b = Rgb::BLACK;

    assert_eq!(a.distance(a), 0);

    assert_eq!(b.distance(a), 30);
    assert_eq!(a.distance(b), 30);
}

#[test]
fn with_brightness_scales_color() {
    let c = Rgb::new(200, 200, 200);

    assert_eq!(c.with_brightness(255), c);
    assert_eq!(c.with_brightness(0), Rgb::BLACK);
    assert_eq!(c.with_brightness(128), Rgb::new(100, 100, 100));
}

#[test]
fn add_sums_channels_and_saturates() {
    assert_eq!(
        Rgb::new(10, 20, 30) + Rgb::new(1, 2, 3),
        Rgb::new(11, 22, 33)
    );

    assert_eq!(
        Rgb::new(200, 0, 0) + Rgb::new(100, 0, 0),
        Rgb::new(255, 0, 0)
    );
}

#[test]
fn from_hue_returns_primary_colors_and_wraps() {
    assert_eq!(Rgb::from_hue(0.0), Rgb::new(255, 0, 0));
    assert_eq!(Rgb::from_hue(120.0), Rgb::new(0, 255, 0));
    assert_eq!(Rgb::from_hue(240.0), Rgb::new(0, 0, 255));
    assert_eq!(Rgb::from_hue(360.0), Rgb::from_hue(0.0));
}

#[test]
fn interpolate_has_expected_length_and_endpoints() {
    let start = Rgb::new(0, 0, 0);
    let end = Rgb::new(10, 20, 30);
    let colors: Vec<_> = Rgb::interpolate(start, end, 10).collect();

    assert_eq!(colors.len(), 3);
    assert_eq!(colors.first().copied(), Some(start));
    assert_eq!(colors.last().copied(), Some(end));
}

#[test]
fn spectrum_conversion_round_trips() {
    let rgb = Rgb::new(12, 34, 56);
    assert_eq!(Rgb::from_spectrum_rgb(rgb.to_spectrum()), rgb);
}

#[test]
fn temperature_conversion_has_reasonable_characteristics() {
    let warm = Rgb::from_temperature_k(2700);
    let neutral = Rgb::from_temperature_k(6500);

    assert!(warm.r > warm.b, "2700K should look warm: {warm:?}");
    assert!(neutral.b > 0, "6500K should contain blue: {neutral:?}");
}
