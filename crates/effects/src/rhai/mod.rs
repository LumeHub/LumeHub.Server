mod color;
mod effects;
mod math;
pub(crate) mod script;

use rhai::Engine;

pub use color::parse_color;

pub fn make_script_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_operations(0);
    math::register(&mut engine);
    color::register(&mut engine);
    effects::register(&mut engine);
    engine
}

pub fn make_signal_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_operations(1_000);
    math::register(&mut engine);
    color::register(&mut engine);
    engine
}
