use bevy::prelude::*;

use crate::components::CoinText;

pub struct CoinUiPlugin;

impl Plugin for CoinUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Coins: ",
                TextStyle {
                    font: asset_server.load("vivaldi.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("vivaldi.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                },
            ),
        ]),
        CoinText,
    ));
}
