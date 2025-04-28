mod items;

use std::{collections::HashMap, io, marker::PhantomData, ops::Deref, sync::Arc};

use bevy::{
    app::Plugin,
    asset::{Asset, AssetApp, AssetLoader, AsyncReadExt, Handle},
    ecs::{bundle::Bundle, component::Component, resource::Resource, system::Commands},
    reflect::{Reflect, TypePath},
};
use items::{Item, ItemRegistryHandle};
use macros::register;
use serde::Deserialize;
use thiserror::Error;

mod macros {
    macro_rules! register {
        ($app:expr, $path:expr, $item:ty, $item_res:ty) => {
            let _: () = {
                fn __load_registry(
                    mut commands: Commands,
                    asset_server: bevy::prelude::Res<bevy::prelude::AssetServer>,
                ) {
                    let handle = asset_server.load(concat!("registry/", $path, ".json"));
                    commands.insert_resource(
                        <$item_res as $crate::registry::FromHandle>::from_handle(handle),
                    );
                }
                $app.add_systems(bevy::app::Startup, __load_registry);
            };

            $app.init_asset::<Registry<$item>>();
            $app.init_asset_loader::<RegistryLoader<Registry<$item>>>();
        };
    }

    macro_rules! impl_from_handle {
        ($type:ty, $reg_item:ty) => {
            impl $crate::registry::FromHandle for $type {
                type RegistryItem = $reg_item;

                fn from_handle(
                    handle: Handle<Registry<<Self as $crate::registry::FromHandle>::RegistryItem>>,
                ) -> Self {
                    Self(handle)
                }
            }
        };
    }

    pub(super) use {impl_from_handle, register};
}

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        register!(app, "items", Item, ItemRegistryHandle);
    }
}

#[derive(Clone, Component, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Identifier(Arc<String>); // todo

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Asset, Default, Reflect)]
pub struct Registry<Item: TypePath + Send + Sync>(HashMap<Identifier, Item>);

#[expect(unused)]
impl<Item> Registry<Item>
where
    Item: Bundle + Clone + TypePath,
{
    pub fn get(&self, identifier: &Identifier) -> Option<Item> {
        self.0.get(identifier).cloned()
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        identifier: &Identifier,
        addition: impl Bundle,
    ) -> bool {
        let item = self.get(identifier);

        if let Some(v) = item {
            commands.spawn((v.clone(), addition));
            return true;
        }

        false
    }
}

struct RegistryLoader<T>(PhantomData<T>);

impl<T: Send + Sync + TypePath> Default for RegistryLoader<Registry<T>> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[derive(Error, Debug)]
pub enum RegistryLoadError {
    #[error("Could not load registry file: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse registry JSON")]
    JsonError(#[from] serde_json::Error),
}

impl<Item: TypePath + Send + Sync + 'static> AssetLoader for RegistryLoader<Registry<Item>>
where
    for<'de> Item: Deserialize<'de>,
{
    type Asset = Registry<Item>;
    type Error = RegistryLoadError;
    type Settings = ();

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _: &Self::Settings,
        _: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await?;
        let map = serde_json::from_str(&buf)?;

        Ok(Registry(map))
    }

    fn extensions(&self) -> &[&str] {
        &["json", "registry.json"]
    }
}

pub trait FromHandle: Resource + Sized {
    type RegistryItem: Send + Sync + TypePath;

    fn from_handle(handle: Handle<Registry<Self::RegistryItem>>) -> Self;
}
