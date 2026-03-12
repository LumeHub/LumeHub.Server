pub mod command;
pub use command::RenderCommand;

use crate::Output;
use domain::{DeviceState, Rgb};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const FRAME_DURATION: Duration = Duration::from_millis(16);

pub fn spawn(mut output: Box<dyn Output>, cmd_rx: Receiver<RenderCommand>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut state = DeviceState {
            on: true,
            brightness: 255,
            color: Rgb::BLACK,
        };
        let mut current: Option<Box<dyn Iterator<Item = Vec<Rgb>> + Send>> = None;

        loop {
            let frame_start = Instant::now();

            loop {
                match cmd_rx.try_recv() {
                    Ok(cmd) => handle_command(cmd, &mut current, &mut state, &mut *output),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }

            if current.is_none() {
                match cmd_rx.recv() {
                    Ok(cmd) => handle_command(cmd, &mut current, &mut state, &mut *output),
                    Err(_) => return,
                }
                continue;
            }

            if let Some(frame) = current.as_mut().and_then(|it| it.next()) {
                if frame.as_slice() != output.pixels() {
                    output.pixels_mut().copy_from_slice(&frame);
                    output.show();
                }
            } else {
                current = None;
            }

            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION {
                thread::sleep(FRAME_DURATION - elapsed);
            }
        }
    })
}

fn handle_command(
    cmd: RenderCommand,
    current: &mut Option<Box<dyn Iterator<Item = Vec<Rgb>> + Send>>,
    state: &mut DeviceState,
    output: &mut dyn Output,
) {
    match cmd {
        RenderCommand::Execute(effect) => {
            *current = Some(effect.frames(output.pixels()));
        }
        RenderCommand::Halt => {
            *current = None;
        }
        RenderCommand::SetBrightness(b) => {
            state.brightness = b;
            if current.is_none() {
                apply_state(state, output);
            }
        }
        RenderCommand::SetOnOff(on) => {
            state.on = on;
            if current.is_none() {
                apply_state(state, output);
            }
        }
        RenderCommand::SetColor(color) => {
            state.color = color;
            *current = None;
            apply_state(state, output);
        }
    }
}

fn apply_state(state: &DeviceState, output: &mut dyn Output) {
    let color = if state.on {
        state.color.with_brightness(state.brightness)
    } else {
        Rgb::BLACK
    };
    let len = output.pixels().len();
    let frame = vec![color; len];
    output.pixels_mut().copy_from_slice(&frame);
    output.show();
}
