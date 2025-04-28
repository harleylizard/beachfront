use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, resource::Resource},
    reflect::TypePath,
};
use serde::Deserialize;

use super::{Registry, macros::impl_from_handle};

#[derive(Resource)]
#[expect(unused)]
pub struct ItemRegistryHandle(Handle<Registry<Item>>);
impl_from_handle!(ItemRegistryHandle, Item);

#[derive(Bundle, Clone, TypePath, Deserialize)]
// TODO #[serde(transparent)]
// TODO add fields for this struct
pub struct Item {}
