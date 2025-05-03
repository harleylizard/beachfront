mod assets;
mod entity;
mod registry;
mod render;

use assets::AssetInitiatorPlugin;
use avian2d::PhysicsPlugins;
use bevy::app::*;
use bevy::prelude::*;
use entity::EntityPlugins;
use entity::movement::MovementPlugin;
use registry::RegistryPlugin;
use render::RenderPlugin;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
#[repr(u8)]
pub enum Stage {
    #[default]
    Initial,
    Game,
    Fail,
}

fn main() {
    let mut app = App::new();
    let default_plugins = DefaultPlugins.set(ImagePlugin::default_nearest());

    app.add_plugins(default_plugins)
        .init_state::<Stage>()
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(MovementPlugin)
        .add_plugins(AssetInitiatorPlugin)
        .add_plugins(RegistryPlugin)
        .add_plugins(RenderPlugin)
        .add_plugins(EntityPlugins);

    #[cfg(debug_assertions)]
    {
        use avian2d::prelude::PhysicsDebugPlugin;
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(PhysicsDebugPlugin::default());
    };

    app.run();
}
