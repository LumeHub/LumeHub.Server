pub mod command;
mod executor;
pub mod transition;

use crate::FrameIter;
type ActiveEffect = Option<FrameIter>;

pub use command::RenderCommand;
pub use executor::spawn;
pub use transition::{blend, fade_to_solid, solid_frames};
