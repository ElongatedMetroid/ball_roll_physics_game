use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Grounded, Jump, MoveSpeed, Player};

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
    player_transform.translation.x -= 200.0;
    player_transform.translation.y -= 100.0;
    player_transform.translation.z += 50.0;

    commands.spawn((
        Name::from("Player"),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: player_transform,
            ..default()
        },
        Player,
        MoveSpeed {
            speed: 300.0,
            break_angular_damping: 100.0,
        },
        Jump {
            height: 500.0,
            amount: 1,
            directional_velocity_multiplier: 100.0,
        },
        RigidBody::Dynamic,
        Collider::ball(10.0),
        GravityScale(5.0),
        Velocity::default(),
        Grounded(false),
        Damping {
            linear_damping: 0.2,
            angular_damping: 0.0,
        },
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut Velocity,
            &mut Transform,
            &mut Grounded,
            &Jump,
            &MoveSpeed,
            &mut Damping,
        ),
        With<Player>,
    >,
) {
    let (mut velocity, mut _position, mut grounded, jump, move_speed, mut damping) =
        player_query.get_single_mut().unwrap();
    let mut moving_right = false;
    let mut moving_left = false;

    // Move right
    if keyboard_input.pressed(KeyCode::D) {
        moving_right = true;
        velocity.linvel.x += move_speed.speed * time.delta().as_secs_f32();
    }
    // Move left
    if keyboard_input.pressed(KeyCode::A) {
        moving_left = true;
        velocity.linvel.x -= move_speed.speed * time.delta().as_secs_f32();
    }
    // Break
    if keyboard_input.pressed(KeyCode::W) {
        if keyboard_input.pressed(KeyCode::LShift) {
            damping.linear_damping = move_speed.break_angular_damping;
        } else {
            damping.angular_damping = move_speed.break_angular_damping;
        }
    }
    if keyboard_input.just_released(KeyCode::W) {
        damping.angular_damping = 0.0;
        damping.linear_damping = 0.2;
    }
    if keyboard_input.just_released(KeyCode::LShift) {
        damping.linear_damping = 0.2;
    }
    // Jump
    if keyboard_input.pressed(KeyCode::Space) && grounded.0 {
        velocity.linvel.y += jump.height;

        if moving_right {
            velocity.linvel.x += (move_speed.speed * time.delta().as_secs_f32())
                * jump.directional_velocity_multiplier;
        } else if moving_left {
            velocity.linvel.x -= (move_speed.speed * time.delta().as_secs_f32())
                * jump.directional_velocity_multiplier;
        }

        grounded.0 = false;
    }
}
