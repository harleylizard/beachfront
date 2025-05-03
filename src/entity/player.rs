use bevy::{
    app::{Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Dir3,
};
use run_once::run_once;

use crate::registry::{Identifier, RegistryAccess};

use super::{
    LivingEntity,
    movement::{Controller, WalkAction},
};

const K_UP: KeyCode = KeyCode::KeyW;
const K_DOWN: KeyCode = KeyCode::KeyS;
const K_RIGHT: KeyCode = KeyCode::KeyA;
const K_LEFT: KeyCode = KeyCode::KeyD;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, (spawn_player, control).chain());
    }
}

#[run_once]
fn spawn_player(mut commands: Commands, reg: RegistryAccess<LivingEntity>) {
    reg.registry()
        .spawn(&mut commands, &Identifier::new("player"), Player::default());
}

#[derive(Component, Default)]
pub struct PlayerMarker;

#[derive(Bundle, Default)]
struct Player {
    _m: PlayerMarker,
}

fn control(
    mut controller: Single<&mut Controller, With<PlayerMarker>>,
    kb: Res<ButtonInput<KeyCode>>,
) {
    let dir = if kb.pressed(K_UP) {
        Some(Dir3::Y)
    } else if kb.pressed(K_DOWN) {
        Some(Dir3::NEG_Y)
    } else if kb.pressed(K_LEFT) {
        Some(Dir3::X)
    } else if kb.pressed(K_RIGHT) {
        Some(Dir3::NEG_X)
    } else {
        None
    };

    controller.action(WalkAction::new(dir, 10., 10.));
}
