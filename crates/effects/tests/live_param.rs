use domain::Rgb;
use effects::bus::ParameterBus;
use effects::live_param::LiveParam;

#[test]
fn set_only_updates_target_not_current() {
    let p = LiveParam::new(Rgb::BLACK, 5.0);
    p.set(Rgb::new(255, 0, 0));
    assert_eq!(p.get(), Rgb::BLACK);
}

#[test]
fn set_immediate_updates_current_without_tick() {
    let p = LiveParam::new(Rgb::BLACK, 5.0);
    p.set_immediate(Rgb::new(255, 0, 0));
    assert_eq!(p.get(), Rgb::new(255, 0, 0));
}

#[test]
fn tick_moves_rgb_toward_target() {
    let p = LiveParam::new(Rgb::BLACK, 5.0);
    let target = Rgb::new(100, 0, 0);
    p.set(target);
    p.tick();
    let after = p.get();
    assert!(after.distance(target) < Rgb::BLACK.distance(target));
}

#[test]
fn tick_reaches_target_for_f32() {
    let p = LiveParam::<f32>::new(0.0, 10.0);
    p.set(100.0);
    for _ in 0..20 {
        p.tick();
    }
    assert_eq!(p.get(), 100.0);
}

#[test]
fn bus_set_color_reaches_target_after_tick() {
    let mut bus = ParameterBus::new(255.0, 255.0);
    bus.register_color("sig".to_string(), Rgb::BLACK, 255.0);
    bus.set_color("sig", Rgb::new(255, 0, 0));
    bus.tick();
    assert_eq!(bus.all_colors()["sig"], Rgb::new(255, 0, 0));
}

#[test]
fn bus_animated_signal_drives_color_via_rhai() {
    let mut bus = ParameterBus::new(255.0, 255.0);
    bus.register_color("sig".to_string(), Rgb::BLACK, 255.0);
    bus.set_animated("sig", "rgb(255, 0, 0)").unwrap();
    bus.tick();
    assert_eq!(bus.all_colors()["sig"], Rgb::new(255, 0, 0));
}

#[test]
fn bus_set_color_after_animated_clears_script() {
    let mut bus = ParameterBus::new(255.0, 255.0);
    bus.register_color("sig".to_string(), Rgb::BLACK, 255.0);
    bus.set_animated("sig", "rgb(255, 0, 0)").unwrap();
    bus.tick();

    bus.set_color("sig", Rgb::new(0, 255, 0));
    for _ in 0..60 {
        bus.tick();
    }
    assert_eq!(bus.all_colors()["sig"], Rgb::new(0, 255, 0));
}

#[test]
fn bus_brightness_advances_via_tick() {
    let bus = ParameterBus::new(0.0, 255.0);
    bus.brightness.set(255.0);
    bus.tick();
    assert_eq!(bus.brightness.get() as u8, 255);
}
