use super::{
    ActiveEffect, RenderCommand,
    transition::{crossfade, transition_to_state},
};
use crate::Output;
use domain::DeviceState;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const FRAME_DURATION: Duration = Duration::from_millis(16);

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
    crossfade_frames: usize,
}

impl Executor {
    fn new(output: Box<dyn Output>, crossfade_frames: usize) -> Self {
        Self {
            output,
            state: DeviceState::default(),
            current: None,
            crossfade_frames,
        }
    }

    fn handle(&mut self, cmd: RenderCommand) {
        match cmd {
            RenderCommand::Execute(effect) => {
                let from = self.output.pixels().to_vec();
                let inner = effect.frames(self.output.pixels());
                self.current = Some(crossfade(from, inner, self.crossfade_frames));
            }
            RenderCommand::Halt => self.current = None,
            RenderCommand::SetBrightness(b) => {
                self.state.brightness = b;
                transition_to_state(
                    &self.state,
                    &mut self.current,
                    &mut *self.output,
                    self.crossfade_frames,
                );
            }
            RenderCommand::SetOnOff(on) => {
                self.state.on = on;
                transition_to_state(
                    &self.state,
                    &mut self.current,
                    &mut *self.output,
                    self.crossfade_frames,
                );
            }
            RenderCommand::SetColor(color) => {
                self.state.color = color;
                self.state.on = true;
                transition_to_state(
                    &self.state,
                    &mut self.current,
                    &mut *self.output,
                    self.crossfade_frames,
                );
            }
        }
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
