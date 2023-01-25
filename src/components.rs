use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MoveSpeed(pub f32);

#[derive(Component)]
pub struct Jump {
    /// Height of jump
    pub height: f32,
    /// Directional Velocity Multiplier, jumping while going a certain direction will boost your velocity in that direction
    /// by MoveSpeed * directional_velocity_multiplier
    pub directional_velocity_multiplier: f32,
    /// Amount of jumps that can be done without being grounded
    pub amount: usize,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
/// Holds a boolean containing whether the entity is grounded or not
pub struct Grounded(pub bool);
