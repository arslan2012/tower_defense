use godot::prelude::*;

mod base_enemy;

struct MainExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MainExtension {}
