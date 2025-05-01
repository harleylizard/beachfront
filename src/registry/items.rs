use bevy::{ecs::bundle::Bundle, reflect::Reflect};
use serde::Deserialize;

use super::SpriteRef;

#[derive(Bundle, Clone, Reflect, Deserialize)]
// TODO #[serde(transparent)]
// TODO add fields for this struct
pub struct Item {
    sprite: SpriteRef,
}
