mod coin;
mod ground;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{coin::CoinPlugin, ground::GroundPlugin};

pub struct ObjectPlugins;

impl PluginGroup for ObjectPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GroundPlugin)
            .add(CoinPlugin)
    }
}
