mod items;

use std::{collections::HashMap, io, marker::PhantomData, ops::Deref, path::PathBuf, sync::Arc};

use bevy::{
    app::Plugin,
    asset::{Asset, AssetApp, AssetLoader, AssetServer, Assets, AsyncReadExt, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        resource::Resource,
        system::{Commands, Local, Query, Res, ResMut, SystemParam},
    },
    image::{Image, TextureAtlas, TextureAtlasLayout},
    log::error,
    math::UVec2,
    reflect::{GetField, Reflect, TypePath},
    sprite::Sprite,
};
use items::Item;
use macros::register;
use serde::Deserialize;
use thiserror::Error;

use crate::{assets::Atlases, render::animation::AnimationConfig};

mod macros {
    macro_rules! register {
        ($app:expr, $path:expr, $item:ty) => {
            let _: () = {
                use bevy::prelude::IntoScheduleConfigs;

                fn __load_registry(
                    mut commands: Commands,
                    asset_server: bevy::prelude::Res<bevy::prelude::AssetServer>,
                ) {
                    let handle: Handle<Registry<$item>> =
                        asset_server.load(concat!("registry/", $path, ".json"));
                    commands.insert_resource(RegistryHandles::new(handle));
                }
                $app.add_systems(bevy::app::PreStartup, __load_registry);
                $app.add_systems(
                    bevy::app::PreUpdate,
                    (
                        Registry::<$item>::create_atlas_handle,
                        Registry::<$item>::update_sprite_refs,
                    )
                        .in_set($crate::assets::RequiresAssetSet),
                );
            };

            $app.init_asset::<Registry<$item>>();
            $app.init_asset_loader::<RegistryLoader<Registry<$item>>>();
        };
    }

    pub(super) use register;
}

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        register!(app, "items", Item);
        #[cfg(debug_assertions)]
        // for inspector debugging
        {
            app.register_type::<SpriteRef>();
            app.register_type::<IndexConfig>();
        }
    }
}

/// Describes the sprite used by a registry item.
#[derive(Clone, Component, Deserialize)]
#[cfg_attr(debug_assertions, derive(Reflect))]
#[cfg_attr(not(debug_assertions), expect(unused))]
pub struct SpriteRef {
    /// Specifies the file to use for the sprite.
    ///
    /// If it is not present, the game will try to use the default sprite.
    file: Option<PathBuf>,
    idx_config: IndexConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(debug_assertions, derive(Reflect))]
#[cfg_attr(not(debug_assertions), expect(unused))]
#[serde(untagged)]
pub enum IndexConfig {
    Single(usize),
    Range(AnimationConfig),
}

#[derive(Clone, Component, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Identifier(Arc<String>); // todo

impl Identifier {
    #[expect(unused)]
    pub fn new(s: impl Into<String>) -> Self {
        Self(Arc::new(s.into()))
    }
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Asset, Reflect, Deserialize)]
pub struct Registry<Item: TypePath + Send + Sync> {
    items: HashMap<Identifier, Item>,
    identifier: String,
    atlas_layout: Option<AtlasConfig>,
}

#[derive(Reflect, Deserialize)]
pub struct AtlasConfig {
    tile_size: (u32, u32),
    columns: u32,
    rows: u32,
}

#[expect(unused)]
impl<Item> Registry<Item>
where
    Item: Bundle + Clone + TypePath,
{
    pub fn get(&self, identifier: &Identifier) -> Option<Item> {
        self.items.get(identifier).cloned()
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        identifier: &Identifier,
        addition: impl Bundle,
    ) -> bool {
        let item = self.get(identifier);

        if let Some(v) = item {
            commands.spawn((v, addition));
            return true;
        }

        false
    }

    fn create_atlas_handle(
        server: Res<AssetServer>,
        registries: Res<Assets<Registry<Item>>>,
        mut handle: ResMut<RegistryHandles<Item>>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        mut run_once: Local<bool>,
    ) {
        if !server.is_loaded(&handle.registry) || *run_once {
            return;
        }

        let registry = registries.get(&handle.registry).unwrap();

        let Some(ref atlas_layout) = registry.atlas_layout else {
            *run_once = true;
            return;
        };

        let atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::from(atlas_layout.tile_size),
            atlas_layout.columns,
            atlas_layout.rows,
            None,
            None,
        );

        let atlas_layout_handle = texture_atlas_layouts.add(atlas_layout);
        handle.atlas_layout = Some(atlas_layout_handle);

        *run_once = true;
    }

    fn update_sprite_refs(
        mut commands: Commands,
        query: Query<(&SpriteRef, Entity)>,
        registry: RegistryAccess<Item>,
        ta_layouts: Res<Assets<TextureAtlasLayout>>,
        atlases: Res<Atlases>,
    ) {
        let Some(tal_handle) = registry.texture_atlas_layout() else {
            panic!("oops");
            return;
        };

        let reg = registry.registry();

        for (spref, id) in &query {
            let Some(image): Option<&Handle<Image>> = atlases.get_field(&reg.identifier) else {
                error!("Could not find texture for registry {}", &reg.identifier);
                return;
            };

            let mut entity_commands = commands.entity(id);

            match spref.idx_config {
                IndexConfig::Single(idx) => {
                    entity_commands.insert(Sprite::from_atlas_image(
                        image.clone_weak(),
                        TextureAtlas {
                            layout: tal_handle.clone_weak(),
                            index: idx,
                        },
                    ));
                }
                IndexConfig::Range(ref ac) => {
                    entity_commands.insert((
                        Sprite::from_atlas_image(
                            image.clone_weak(),
                            TextureAtlas {
                                layout: tal_handle.clone_weak(),
                                index: ac.start,
                            },
                        ),
                        ac.clone(),
                    ));
                }
            };

            entity_commands.remove::<SpriteRef>();
        }
    }
}

struct RegistryLoader<T>(PhantomData<T>);

impl<T: Send + Sync + TypePath> Default for RegistryLoader<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[derive(Error, Debug)]
pub enum RegistryLoadError {
    #[error("Could not load registry file: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse registry JSON: {0}")]
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
        let mut buf = String::with_capacity(1024 * 64); // 64K
        reader.read_to_string(&mut buf).await?;
        let s = serde_json::from_str(&buf)?;

        Ok(s)
    }

    fn extensions(&self) -> &[&str] {
        &["json", "registry.json"]
    }
}

#[derive(Resource)]
pub struct RegistryHandles<Item: Send + Sync + TypePath> {
    registry: Handle<Registry<Item>>,
    atlas_layout: Option<Handle<TextureAtlasLayout>>,
}

impl<Item> RegistryHandles<Item>
where
    Item: Send + Sync + TypePath,
{
    const fn new(inner: Handle<Registry<Item>>) -> Self {
        Self {
            registry: inner,
            atlas_layout: None,
        }
    }
}

#[derive(SystemParam)]
pub struct RegistryAccess<'w, Item>
where
    Item: Send + Sync + TypePath,
{
    handle: Res<'w, RegistryHandles<Item>>,
    registries: Res<'w, Assets<Registry<Item>>>,
}

impl<'w, Item> RegistryAccess<'w, Item>
where
    Item: Send + Sync + TypePath,
{
    pub fn registry(&self) -> &Registry<Item> {
        self.registries.get(&self.handle.registry).unwrap()
    }

    fn texture_atlas_layout(&self) -> &Option<Handle<TextureAtlasLayout>> {
        &self.handle.atlas_layout
    }
}
