use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use platformer::{camera::PlayerCameraPlugin, level::LevelPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(PlayerCameraPlugin)
        .add_plugin(LevelPlugin)
        .run();
}
