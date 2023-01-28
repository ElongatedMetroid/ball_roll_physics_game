use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Coin, CoinText, Player};

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(check_collected);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("ferris.png");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            texture: texture_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.0)),
                ..default()
            },
            ..default()
        },
        Collider::ball(50.0),
        Coin(1),
    ));
}

fn check_collected(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut coin_text: Query<&mut Text, With<CoinText>>,
    player_query: Query<Entity, With<Player>>,
    coin_query: Query<(&Coin, Entity)>,
) {
    let player_entity = player_query.get_single().unwrap();
    let mut coin_text = coin_text.get_single_mut().unwrap();

    for (coin, coin_entity) in &coin_query {
        if let Some(contact_pair) = rapier_context.contact_pair(coin_entity, player_entity) {
            if contact_pair.has_any_active_contacts() {
                // Add coin value to coin counter
                let current_coins = coin_text.sections[1].value.parse::<isize>().unwrap_or_default();
                coin_text.sections[1].value = format!("{}", current_coins + coin.0);

                // Despawn coin
                commands.entity(coin_entity).despawn_recursive();
            }
        }
    }
}
