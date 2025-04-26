mod assets;
mod entity;

use assets::AssetInitiatorPlugin;
use bevy::DefaultPlugins;
use bevy::app::*;
use bevy::prelude::*;
use discriminant::Enum;

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
        .add_plugins(AssetInitiatorPlugin);

    app.run();
}
