use bevy::prelude::*;

use crate::components::Player;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(follow_player);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn follow_player(
    player_position: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_position: Query<&mut Transform, With<Camera>>,
) {
    let player_position = player_position.get_single().unwrap();
    let mut camera_position = camera_position.get_single_mut().unwrap();

    camera_position.translation = player_position.translation;
}
