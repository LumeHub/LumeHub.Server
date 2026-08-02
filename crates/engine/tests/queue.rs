use domain::{Rgb, TransitionCurve, TransitionSpec};
use engine::{Effect, EffectQueue, FrameIter, RenderCommand};

struct Noop;
impl Effect for Noop {
    fn frames(&self, _: &[Rgb]) -> FrameIter {
        Box::new(std::iter::empty())
    }
}

fn spec(duration_ms: u32) -> TransitionSpec {
    TransitionSpec::new(duration_ms, TransitionCurve::Linear)
}

#[test]
fn brightness_speed_is_proportional_to_default_fade_duration() {
    let (fast, _) = EffectQueue::new(spec(160));
    let (slow, _) = EffectQueue::new(spec(1600));
    assert!(fast.brightness_speed() > slow.brightness_speed());
}

#[test]
fn zero_fade_duration_gives_max_brightness_speed() {
    let (q, _) = EffectQueue::new(TransitionSpec::INSTANT);
    assert_eq!(q.brightness_speed(), 255.0);
}

#[test]
fn enqueue_sends_execute_command_with_default_spec() {
    let default = spec(320);
    let (q, rx) = EffectQueue::new(default);
    q.enqueue(Box::new(Noop));
    match rx.try_recv().unwrap() {
        RenderCommand::Execute(_, s) => assert_eq!(s, default),
        _ => panic!("expected Execute command"),
    }
}

#[test]
fn enqueue_with_overrides_default_spec() {
    let (q, rx) = EffectQueue::new(spec(0));
    let override_spec = TransitionSpec::new(500, TransitionCurve::EaseOut);
    q.enqueue_with(Box::new(Noop), override_spec);
    match rx.try_recv().unwrap() {
        RenderCommand::Execute(_, s) => assert_eq!(s, override_spec),
        _ => panic!("expected Execute command"),
    }
}

#[test]
fn set_color_emits_command_with_default_spec() {
    let default = spec(80);
    let (q, rx) = EffectQueue::new(default);
    q.set_color(Rgb::new(10, 20, 30));
    match rx.try_recv().unwrap() {
        RenderCommand::SetColor(color, s) => {
            assert_eq!(color, Rgb::new(10, 20, 30));
            assert_eq!(s, default);
        }
        _ => panic!("expected SetColor command"),
    }
}

#[test]
fn set_color_with_threads_caller_spec_through() {
    let (q, rx) = EffectQueue::new(spec(0));
    let custom = TransitionSpec::new(800, TransitionCurve::EaseInOut);
    q.set_color_with(Rgb::new(1, 2, 3), custom);
    match rx.try_recv().unwrap() {
        RenderCommand::SetColor(_, s) => assert_eq!(s, custom),
        _ => panic!("expected SetColor command"),
    }
}

#[test]
fn halt_emits_halt_command() {
    let (q, rx) = EffectQueue::new(spec(0));
    q.halt();
    assert!(matches!(rx.try_recv().unwrap(), RenderCommand::Halt));
}

#[test]
fn default_spec_returns_constructor_value() {
    let s = spec(420);
    let (q, _) = EffectQueue::new(s);
    assert_eq!(q.default_spec(), s);
}
