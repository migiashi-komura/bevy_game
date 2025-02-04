mod camera;
mod characters;
mod light;

use bevy::prelude::*;
use camera::CameraPlugin;
use characters::CharactersPlugin;
use light::LightPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CharactersPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .run();
}
