use crate::textures::Atlases;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
}

#[derive(Bundle)]
pub struct Item {
    position: Position,
    transform: Transform
}

impl Position {
    pub fn new() -> Self {
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
        }
    }
}

impl Item {
    pub fn new() -> Item {
        Item {
            position: Position::new(),
            transform: Transform::default()
        }
    }
}

impl Move for Item {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn translation(&mut self) -> &mut Vec3 {
        &mut self.transform.translation
    }
}

trait Move {
    fn position(&mut self) -> &mut Position;

    fn translation(&mut self) -> &mut Vec3;

    fn moves(&mut self, x: f32, y: f32, z: f32) {
        let position = self.position();
        position.velocity_x += x;
        position.velocity_y += y;
        position.velocity_z += z;
    }

    fn step(&mut self, delta_time: f32) {
        let drag = 0.1;

        let position = self.position();
        position.velocity_x *= drag;
        position.velocity_y *= drag;
        position.velocity_z *= drag;

        position.x += position.velocity_x * delta_time;
        position.y += position.velocity_y * delta_time;
        position.z += position.velocity_z * delta_time;

        let (x, y, z) = (position.x, position.y, position.z);

        let translation = self.translation();
        translation.x = x;
        translation.y = z + y;
        translation.z = 0.0;
    }
}

pub fn draw_entity(mut commands: Commands, query: Query<(Entity, &Position), With<Position>>, atlases: Res<Atlases>) {
}

pub fn step_entity() {
}