mod entity;
mod textures;

use crate::entity::{draw_entity, Item};
use crate::textures::Textures;
use bevy::app::*;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum Stage {
  #[default]
  Loading,
  Game
}

fn main() {
  let mut app = App::new();
  app.add_plugins(DefaultPlugins.set(ImagePlugin::default()));
  app.insert_state(Stage::Loading);
  app.add_loading_state(
    LoadingState::new(Stage::Loading)
        .continue_to_state(Stage::Game)
        .load_collection::<Textures>()
  );
  app.add_systems(Startup, start_up);
  app.add_systems(Update, update);
  app.add_systems(Update, draw_entity.run_if(in_state(Stage::Game)));
  app.run();
}

fn start_up(mut commands: Commands) {
  commands.spawn(Camera2d);
  commands.spawn(Item::new());
}

fn update() {

}