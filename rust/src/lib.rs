use godot::prelude::*;

mod base_enemy;
mod camera;
mod map;

struct MainExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MainExtension {
    fn on_level_init(_level: InitLevel) {
        godot_print!("Hello, world!"); // Prints to the Godot console
    }
}
