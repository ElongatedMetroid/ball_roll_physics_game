//! Handles the level including loading, and the objects
//! Z-Ordering
//! Background 1 - 0.0
//! Background 2 - 1.0
//! Background 3 - 2.0
//! Foreground Objects (Coins, Ground, etc.) - 3.0
//! ------- other stuff? -------
//! Player - 50.0

use bevy::prelude::*;

use crate::objects::ObjectPlugins;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ObjectPlugins).add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("trees.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

// Loads level from file
// fn load_level() {

// }
