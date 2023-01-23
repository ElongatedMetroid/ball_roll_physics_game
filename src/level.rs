use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("trees.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 100.0, 100.0),
                custom_size: Some(Vec2::new(200.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -300.0, 10.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(100.0, 25.0),
    ));
}
