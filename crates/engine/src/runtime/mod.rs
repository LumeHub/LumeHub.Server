pub mod command;
mod executor;
mod transition;

use crate::FrameIter;
type ActiveEffect = Option<FrameIter>;

pub use command::RenderCommand;
pub use executor::spawn;
