pub mod movement;
mod player;

use avian2d::prelude::RigidBody;
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    ecs::bundle::Bundle,
    reflect::Reflect,
};
use movement::Controller;
use player::PlayerPlugin;
use serde::Deserialize;

use crate::registry::SpriteRef;

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PlayerPlugin)
    }
}

#[derive(Bundle, Deserialize, Reflect)]
pub struct LivingEntity {
    sprite: SpriteRef,
    #[serde(skip)]
    #[serde(default)]
    controller: Controller,
    #[serde(skip)]
    #[serde(default)]
    rb: RigidBody,
}

impl Clone for LivingEntity {
    fn clone(&self) -> Self {
        Self {
            sprite: self.sprite.clone(),
            controller: Controller::default(),
            rb: RigidBody::default(),
        }
    }
}
