use bevy::prelude::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("ferris.png");

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 3.0),
        texture: texture_handle,
        ..default()
    });
}
