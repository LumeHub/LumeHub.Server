use crate::Effect;
use domain::{Rgb, TransitionSpec};

pub enum RenderCommand {
    Execute(Box<dyn Effect + Send>, TransitionSpec),
    SetBrightness(u8, TransitionSpec),
    SetOnOff(bool, TransitionSpec),
    SetColor(Rgb, TransitionSpec),
    Halt,
}
