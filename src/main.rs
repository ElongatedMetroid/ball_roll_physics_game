use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use platformer::{
    camera::PlayerCameraPlugin, level::LevelPlugin, player::PlayerPlugin, ui::debug::DebugUiPlugins,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PlayerPlugin)
        .add_plugin(PlayerCameraPlugin)
        .add_plugin(LevelPlugin);

    if cfg!(debug_assertions) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(WorldInspectorPlugin)
            .add_plugins(DebugUiPlugins);
    }

    app.run()
}
