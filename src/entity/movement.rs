use avian2d::prelude::LinearVelocity;
use bevy::{
    app::{FixedPostUpdate, Plugin},
    ecs::{component::Component, system::Res},
    math::{Dir3, Vec2},
    prelude::Query,
    reflect::Reflect,
    time::Time,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(FixedPostUpdate, move_components);
    }
}

#[derive(Component, Default, Reflect)]
pub struct Controller {
    #[reflect(ignore)]
    action: Option<Box<dyn Action>>,
}

impl Controller {
    #[allow(private_bounds)]
    pub fn action<A>(&mut self, action: A)
    where
        A: Action,
    {
        self.action.replace(Box::new(action));
    }

    fn apply(&mut self, linear_velocity: &mut LinearVelocity, delta: f32) {
        self.action
            .as_ref()
            .inspect(|a| {
                let direction_vector = a
                    .direction()
                    .map(|d| d.as_vec3())
                    .unwrap_or_default()
                    .truncate();

                let speed_sqr = linear_velocity.0.length_squared();
                let max_speed_sqr = a.speed() * a.speed();

                // Decelerate if the direction is [0, 0] or if the current speed is higher than the max speed specified by the struct
                if direction_vector == Vec2::ZERO && speed_sqr > 0. || speed_sqr >= max_speed_sqr {
                    let velocity_offset = delta * a.acceleration();

                    linear_velocity.0.x += velocity_offset
                        * match linear_velocity.0.x < 0. {
                            true => 1.,
                            false => -1.,
                        };

                    linear_velocity.0.y += velocity_offset
                        * match linear_velocity.0.y < 0. {
                            true => 1.,
                            false => -1.,
                        };
                } else {
                    linear_velocity.0 += direction_vector * delta * a.acceleration();
                }
            })
            .take();
    }
}

trait Action: Send + Sync + 'static {
    fn speed(&self) -> f32;
    fn acceleration(&self) -> f32;
    fn direction(&self) -> Option<Dir3>;
}

pub struct WalkAction {
    pub direction: Option<Dir3>,
    pub speed: f32,
    pub acceleration: f32,
}

impl WalkAction {
    pub const fn new(direction: Option<Dir3>, speed: f32, acceleration: f32) -> Self {
        Self {
            direction,
            speed,
            acceleration,
        }
    }
}

impl Action for WalkAction {
    fn acceleration(&self) -> f32 {
        self.acceleration
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn direction(&self) -> Option<Dir3> {
        self.direction
    }
}

pub(super) fn move_components(
    mut query: Query<(&mut LinearVelocity, &mut Controller)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut linear_v, mut controller) in &mut query {
        controller.apply(&mut linear_v, delta);
    }
}
