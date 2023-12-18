use godot::prelude::*;

mod main_scene;
mod shapell;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
