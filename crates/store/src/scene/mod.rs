mod active;
mod layers;
mod scenes;

pub const ACTIVE_SCENE_ID: &str = "__active__";

pub use active::{
    load_into_active, overwrite_from_active, replace_active_layers, restore_active_from_stack,
    save_active_as,
};
pub use layers::{
    LayerRecord, NewLayer, add_layer, clear_layers, get_layer, get_layer_in_scene, get_layers,
    remove_layer, reorder_layers, update_layer,
};
pub use scenes::{SceneRecord, create, delete, get_all, get_one, update};
