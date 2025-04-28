mod assets;
mod entity;
mod registry;

use assets::AssetInitiatorPlugin;
use bevy::DefaultPlugins;
use bevy::app::*;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use discriminant::Enum;
use registry::RegistryPlugin;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States, Enum)]
#[repr(u8)]
pub enum Stage {
    #[default]
    Initial,
    Game,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AssetInitiatorPlugin)
        .add_plugins(RegistryPlugin);

    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::default(),
        ))
    };

    app.run();
}
