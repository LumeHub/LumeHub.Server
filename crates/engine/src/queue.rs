use std::sync::mpsc;

use domain::{Rgb, TransitionSpec};

use crate::{Effect, FRAME_DURATION_MS, RenderCommand};

#[derive(Clone)]
pub struct EffectQueue {
    sender: mpsc::Sender<RenderCommand>,
    default_spec: TransitionSpec,
}

impl EffectQueue {
    pub fn new(default_spec: TransitionSpec) -> (Self, mpsc::Receiver<RenderCommand>) {
        let (sender, receiver) = mpsc::channel();
        (
            Self {
                sender,
                default_spec,
            },
            receiver,
        )
    }

    pub fn default_spec(&self) -> TransitionSpec {
        self.default_spec
    }

    pub fn brightness_speed(&self) -> f32 {
        255.0 / self.default_frames().max(1) as f32
    }

    pub fn color_speed(&self) -> f32 {
        255.0 / self.default_frames().max(1) as f32
    }

    pub fn enqueue(&self, effect: Box<dyn Effect + Send>) {
        self.enqueue_with(effect, self.default_spec);
    }

    pub fn enqueue_with(&self, effect: Box<dyn Effect + Send>, spec: TransitionSpec) {
        self.dispatch(RenderCommand::Execute(effect, spec));
    }

    pub fn set_color(&self, color: Rgb) {
        self.set_color_with(color, self.default_spec);
    }

    pub fn set_color_with(&self, color: Rgb, spec: TransitionSpec) {
        self.dispatch(RenderCommand::SetColor(color, spec));
    }

    pub fn set_brightness(&self, brightness: u8) {
        self.set_brightness_with(brightness, self.default_spec);
    }

    pub fn set_brightness_with(&self, brightness: u8, spec: TransitionSpec) {
        self.dispatch(RenderCommand::SetBrightness(brightness, spec));
    }

    pub fn set_on_off(&self, on: bool) {
        self.set_on_off_with(on, self.default_spec);
    }

    pub fn set_on_off_with(&self, on: bool, spec: TransitionSpec) {
        self.dispatch(RenderCommand::SetOnOff(on, spec));
    }

    pub fn halt(&self) {
        self.dispatch(RenderCommand::Halt);
    }

    pub fn send(&self, cmd: RenderCommand) {
        self.dispatch(cmd);
    }

    fn dispatch(&self, cmd: RenderCommand) {
        self.sender.send(cmd).unwrap();
    }

    fn default_frames(&self) -> u32 {
        self.default_spec.frames(FRAME_DURATION_MS)
    }
}
