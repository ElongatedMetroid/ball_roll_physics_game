use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Ground, Grounded};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(check_grounded);
    }
}

fn setup(mut commands: Commands) {
    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 100.0, 100.0),
                custom_size: Some(Vec2::new(100_000_000_000.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -300.0, 2.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50_000_000_000.0, 25.0),
        Ground,
    ));

    let mut slope_transform = Transform::from_xyz(0.0, -300.0, 2.0);
    slope_transform.rotate_z(340.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 100.0, 100.0),
                custom_size: Some(Vec2::new(200.0, 50.0)),
                ..default()
            },
            transform: slope_transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(100.0, 25.0),
        Ground,
        ActiveEvents::COLLISION_EVENTS,
    ));

    let mut slope_transform = Transform::from_xyz(1000.0, -300.0, 2.0);
    slope_transform.rotate_z(500.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 100.0, 100.0),
                custom_size: Some(Vec2::new(2000.0, 50.0)),
                ..default()
            },
            transform: slope_transform,
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
        let mut no_collision = true;

        for ground in &ground_query {
            if let Some(contact_pair) = rapier_context.contact_pair(other_entity, ground) {
                no_collision = false;
                if contact_pair.has_any_active_contacts() {
                    grounded.0 = true;

                    break;
                }
            }
        }

        // If the entity has no collisions make sure to set grounded to false since without doing this, if the entity fell off an edge their
        // grounded would still be set to true, and for example if this is the player, the player would be able to jump even though they
        // are in the air.
        if no_collision {
            grounded.0 = false;
        }
    }
}
