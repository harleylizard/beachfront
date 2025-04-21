use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 16, tile_size_y = 16, columns = 8, rows = 8))]
    #[asset(path = "textures/items.png")]
    items: Handle<Image>
}