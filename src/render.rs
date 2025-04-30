use bevy::{
    app::{Plugin, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_camera);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
