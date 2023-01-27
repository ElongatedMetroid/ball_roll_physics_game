use bevy::{app::PluginGroupBuilder, prelude::*};

use self::fps::FpsUiPlugin;

mod fps;

pub struct DebugUiPlugins;

impl PluginGroup for DebugUiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(FpsUiPlugin)
    }
}
