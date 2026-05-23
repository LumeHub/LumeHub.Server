use super::{
    ActiveEffect, RenderCommand,
    transition::{blend, fade_to_solid, solid_frames},
};
use crate::Output;
use domain::{DeviceState, Rgb, TransitionCurve};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const FRAME_DURATION: Duration = Duration::from_millis(16);
const DEFAULT_CURVE: TransitionCurve = TransitionCurve::Linear;

pub fn spawn(
    output: Box<dyn Output>,
    cmd_rx: Receiver<RenderCommand>,
    crossfade_frames: usize,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut ex = Executor::new(output, crossfade_frames);

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
    crossfade_frames: u32,
}

impl Executor {
    fn new(output: Box<dyn Output>, crossfade_frames: usize) -> Self {
        Self {
            output,
            state: DeviceState::default(),
            current: None,
            crossfade_frames: crossfade_frames as u32,
        }
    }

    fn handle(&mut self, cmd: RenderCommand) {
        match cmd {
            RenderCommand::Execute(effect) => {
                let to = effect.frames(self.output.pixels());
                let from = self.take_from();
                self.current = Some(blend(from, to, self.crossfade_frames, DEFAULT_CURVE));
            }
            RenderCommand::Halt => self.current = None,
            RenderCommand::SetBrightness(b) => {
                self.state.brightness = b;
                self.transition_to_state();
            }
            RenderCommand::SetOnOff(on) => {
                self.state.on = on;
                self.transition_to_state();
            }
            RenderCommand::SetColor(color) => {
                self.state.color = color;
                self.state.on = true;
                self.transition_to_state();
            }
        }
    }

    fn transition_to_state(&mut self) {
        let target = self.target_pixels();
        if self.crossfade_frames == 0 {
            self.output.pixels_mut().copy_from_slice(&target);
            self.output.show();
            self.current = None;
            return;
        }
        let from = self.take_from();
        self.current = Some(fade_to_solid(
            from,
            target,
            self.crossfade_frames,
            DEFAULT_CURVE,
        ));
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
