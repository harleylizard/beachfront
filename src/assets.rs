use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::Stage;

pub struct AssetInitiatorPlugin;

impl Plugin for AssetInitiatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(Stage::Initial)
                .continue_to_state(Stage::Game)
                .load_collection::<Atlases>(),
        )
        .configure_sets(Update, RequiresAssetSet.run_if(in_state(Stage::Game)))
        .configure_sets(FixedUpdate, RequiresAssetSet.run_if(in_state(Stage::Game)));
    }
}

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RequiresAssetSet;

#[derive(AssetCollection, Resource)]
pub struct Atlases {
    #[asset(texture_atlas(tile_size_x = 16, tile_size_y = 16, columns = 8, rows = 8))]
    #[asset(path = "textures/items.png")]
    pub items: Handle<Image>,
}
