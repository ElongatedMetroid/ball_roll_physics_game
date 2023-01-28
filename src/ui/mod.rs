pub mod coin;
pub mod debug;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::coin::CoinUiPlugin;

pub struct GameUiPlugins;

impl PluginGroup for GameUiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(CoinUiPlugin)
    }
}
