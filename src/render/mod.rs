pub mod animation;

use bevy::{
    app::{FixedUpdate, Plugin, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
    math::Vec3,
    transform::components::Transform,
};

const SCALE: f32 = 1. / 5.;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_camera);
        app.add_systems(FixedUpdate, animation::execute_animations);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_scale(Vec3::splat(SCALE))));
}
