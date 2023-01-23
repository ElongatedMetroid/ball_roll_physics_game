use bevy::prelude::*;
use platformer::{camera::PlayerCameraPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(PlayerCameraPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("trees.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
