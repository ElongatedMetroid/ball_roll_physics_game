use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use platformer::{
    camera::PlayerCameraPlugin,
    level::LevelPlugin,
    player::PlayerPlugin,
    ui::{debug::DebugUiPlugins, GameUiPlugins},
};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // This tells the AssetServer to watch for changes to assets.
                // It enables our scenes to automatically reload in game when we modify their files.
                watch_for_changes: true,
                ..default()
            })
            .set(ImagePlugin::default_nearest())
    )
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(PlayerPlugin)
    .add_plugin(PlayerCameraPlugin)
    .add_plugin(LevelPlugin)
    .add_plugins(GameUiPlugins);

    if cfg!(debug_assertions) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(WorldInspectorPlugin)
            .add_plugins(DebugUiPlugins);
    }

    app.run()
}
