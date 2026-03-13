use domain::Rgb;
use engine::{Effect, EffectQueue, FrameIter, RenderCommand};

struct Noop;
impl Effect for Noop {
    fn frames(&self, _: &[Rgb]) -> FrameIter {
        Box::new(std::iter::empty())
    }
}

#[test]
fn brightness_speed_is_proportional_to_crossfade_ms() {
    let (fast, _) = EffectQueue::new(160);
    let (slow, _) = EffectQueue::new(1600);
    assert!(fast.brightness_speed() > slow.brightness_speed());
}

#[test]
fn zero_crossfade_gives_max_brightness_speed() {
    let (q, _) = EffectQueue::new(0);
    assert_eq!(q.brightness_speed(), 255.0);
}

#[test]
fn enqueue_sends_execute_command() {
    let (q, rx) = EffectQueue::new(0);
    q.enqueue(Box::new(Noop));
    assert!(matches!(rx.try_recv().unwrap(), RenderCommand::Execute(_)));
}

#[test]
fn send_delivers_command_directly() {
    let (q, rx) = EffectQueue::new(0);
    q.send(RenderCommand::Halt);
    assert!(matches!(rx.try_recv().unwrap(), RenderCommand::Halt));
}
