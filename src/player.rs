use bevy::prelude::*;

use crate::components::{MoveSpeed, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(move_player);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut player_transform = Transform::from_scale(Vec3::splat(6.0));
    player_transform.translation.z += 500.0;

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: player_transform,
            ..default()
        },
        Player,
        MoveSpeed(5.0),
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    let (mut position, move_speed) = player_query.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::A) {
        position.translation.x -= move_speed.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        position.translation.x += move_speed.0;
    }
}
