pub mod animation;

use bevy::{
    app::{FixedUpdate, Plugin, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_camera);
        app.add_systems(FixedUpdate, animation::execute_animations);
    }
}

fn init_camera(mut commands: Commands) {
    println!("abc");
    commands.spawn(Camera2d);
}
