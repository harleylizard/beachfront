use bevy::{
    app::{App, FixedUpdate, Plugin, PreUpdate, Update},
    asset::Handle,
    ecs::{
        resource::Resource,
        schedule::{IntoScheduleConfigs, SystemSet},
    },
    image::Image,
    reflect::Reflect,
    state::condition::in_state,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};

use crate::Stage;

pub struct AssetInitiatorPlugin;

impl Plugin for AssetInitiatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(Stage::Initial)
                .continue_to_state(Stage::Game)
                .on_failure_continue_to_state(Stage::Fail)
                .load_collection::<Atlases>(),
        )
        .configure_sets(Update, RequiresAssetSet.run_if(in_state(Stage::Game)))
        .configure_sets(PreUpdate, RequiresAssetSet.run_if(in_state(Stage::Game)))
        .configure_sets(FixedUpdate, RequiresAssetSet.run_if(in_state(Stage::Game)));
    }
}

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RequiresAssetSet;

#[derive(AssetCollection, Resource, Reflect)]
pub struct Atlases {
    #[asset(path = "textures/items.png")]
    pub items: Handle<Image>,
}
