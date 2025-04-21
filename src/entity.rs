use crate::textures::Textures;
use bevy::prelude::{Bundle, Commands, Component, Entity, Query, Res, With};

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Bundle)]
pub struct Item {
    position: Position
}

impl Item {
    pub fn new() -> Item {
        Item {
            position: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}

pub fn draw_entity(mut commands: Commands, query: Query<(Entity, &Position), With<Position>>, textures: Res<Textures>) { 
    for (entity, position) in query.iter() {
    }
}