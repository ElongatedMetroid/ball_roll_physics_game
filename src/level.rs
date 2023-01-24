use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Ground, Grounded};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(check_grounded);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("trees.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 100.0, 100.0),
                custom_size: Some(Vec2::new(2000.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -300.0, 10.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(1000.0, 25.0),
        Ground,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn check_grounded(
    rapier_context: Res<RapierContext>,
    // Query for objects that can be grounded
    mut entity_query: Query<(&mut Grounded, Entity)>,
    ground_query: Query<Entity, With<Ground>>,
) {
    for (mut grounded, other_entity) in &mut entity_query {
        for ground in &ground_query {
            if let Some(contact_pair) = rapier_context.contact_pair(other_entity, ground) {
                if contact_pair.has_any_active_contacts() {
                    grounded.0 = true;
                }
            }
        }
    }
}
