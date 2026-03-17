use effects::LiveParam;

#[test]
fn set_only_updates_target_not_current() {
    let p = LiveParam::new(0.0f32, 5.0);
    p.set(100.0);
    assert_eq!(p.get(), 0.0);
}

#[test]
fn set_immediate_updates_current_without_tick() {
    let p = LiveParam::new(0.0f32, 5.0);
    p.set_immediate(100.0);
    assert_eq!(p.get(), 100.0);
}

#[test]
fn tick_moves_f32_toward_target() {
    let p = LiveParam::new(0.0f32, 10.0);
    p.set(100.0);
    p.tick();
    let after = p.get();
    assert!(after > 0.0 && after <= 100.0);
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
