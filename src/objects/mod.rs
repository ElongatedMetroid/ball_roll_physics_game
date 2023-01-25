mod coin;
mod ground;

use bevy::prelude::*;

use self::{coin::CoinPlugin, ground::GroundPlugin};

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GroundPlugin).add_plugin(CoinPlugin);
    }
}
