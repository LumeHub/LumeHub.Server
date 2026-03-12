use crate::Effect;
use domain::Rgb;

pub enum RenderCommand {
    Execute(Box<dyn Effect + Send>),
    SetBrightness(u8),
    SetOnOff(bool),
    SetColor(Rgb),
    Halt,
}
