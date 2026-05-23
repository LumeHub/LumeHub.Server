pub mod effect;
pub use effect::{Effect, FrameIter};
pub mod runtime;
pub use runtime::command::RenderCommand;
pub use runtime::{blend, fade_to_solid, solid_frames, spawn};
pub mod output;
pub use output::Output;
pub mod queue;
pub use queue::EffectQueue;

pub const FRAME_DURATION_MS: u32 = 16;
