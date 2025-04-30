use bevy::{ecs::bundle::Bundle, reflect::TypePath};
use serde::Deserialize;

use super::SpriteRef;

#[derive(Bundle, Clone, TypePath, Deserialize)]
// TODO #[serde(transparent)]
// TODO add fields for this struct
pub struct Item {
    sprite: SpriteRef,
}
