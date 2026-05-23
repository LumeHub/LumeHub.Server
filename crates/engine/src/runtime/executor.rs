use super::{
    ActiveEffect, RenderCommand,
    transition::{blend, fade_to_solid, solid_frames},
};
use crate::{FRAME_DURATION_MS, Output};
use domain::{DeviceState, Rgb, TransitionSpec};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const FRAME_DURATION: Duration = Duration::from_millis(FRAME_DURATION_MS as u64);

pub fn spawn(output: Box<dyn Output>, cmd_rx: Receiver<RenderCommand>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut ex = Executor::new(output);

        loop {
            let frame_start = Instant::now();

            loop {
                match cmd_rx.try_recv() {
                    Ok(cmd) => ex.handle(cmd),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }

            if ex.current.is_none() {
                match cmd_rx.recv() {
                    Ok(cmd) => ex.handle(cmd),
                    Err(_) => return,
                }
                continue;
            }

            ex.tick();

            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION {
                thread::sleep(FRAME_DURATION - elapsed);
            }
        }
    })
}

struct Executor {
    output: Box<dyn Output>,
    state: DeviceState,
    current: ActiveEffect,
}

impl Executor {
    fn new(output: Box<dyn Output>) -> Self {
        Self {
            output,
            state: DeviceState::default(),
            current: None,
        }
    }

    fn handle(&mut self, cmd: RenderCommand) {
        match cmd {
            RenderCommand::Execute(effect, spec) => {
                let to = effect.frames(self.output.pixels());
                let from = self.take_from();
                self.current = Some(blend(from, to, spec.frames(FRAME_DURATION_MS), spec.curve));
            }
            RenderCommand::Halt => self.current = None,
            RenderCommand::SetBrightness(b, spec) => {
                self.state.brightness = b;
                self.transition_to_state(spec);
            }
            RenderCommand::SetOnOff(on, spec) => {
                self.state.on = on;
                self.transition_to_state(spec);
            }
            RenderCommand::SetColor(color, spec) => {
                self.state.color = color;
                self.state.on = true;
                self.transition_to_state(spec);
            }
        }
    }

    fn transition_to_state(&mut self, spec: TransitionSpec) {
        let target = self.target_pixels();
        let frames = spec.frames(FRAME_DURATION_MS);
        if frames == 0 {
            self.output.pixels_mut().copy_from_slice(&target);
            self.output.show();
            self.current = None;
            return;
        }
        let from = self.take_from();
        self.current = Some(fade_to_solid(from, target, frames, spec.curve));
    }

    fn take_from(&mut self) -> crate::FrameIter {
        self.current
            .take()
            .unwrap_or_else(|| solid_frames(self.output.pixels().to_vec()))
    }

    fn target_pixels(&self) -> Vec<Rgb> {
        let color = if self.state.on {
            self.state.color.with_brightness(self.state.brightness)
        } else {
            Rgb::BLACK
        };
        vec![color; self.output.pixels().len()]
    }

    fn tick(&mut self) {
        if let Some(frame) = self.current.as_mut().and_then(|it| it.next()) {
            if frame.as_slice() != self.output.pixels() {
                self.output.pixels_mut().copy_from_slice(&frame);
                self.output.show();
            }
        } else {
            self.current = None;
        }
    }
}
