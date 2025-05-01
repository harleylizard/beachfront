use std::time::Duration;

use bevy::{
    ecs::{
        component::Component,
        system::{Query, Res},
    },
    reflect::Reflect,
    sprite::Sprite,
    time::{Time, Timer},
};
use serde::Deserialize;

// taken from the bevy examples (https://bevyengine.org/examples/2d-rendering/sprite-animation)

#[derive(Clone, Component, Debug, Reflect)]
pub struct AnimationConfig {
    pub start: usize,
    pub stop: usize,
    pub fps: u8,
    pub timer: Timer,
}

impl<'de> Deserialize<'de> for AnimationConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Partial {
            start: usize,
            stop: usize,
            fps: u8,
        }

        let Partial { start, stop, fps } = Partial::deserialize(deserializer)?;
        Ok(Self::new(start, stop, fps))
    }
}

impl AnimationConfig {
    pub fn new(start: usize, stop: usize, fps: u8) -> Self {
        Self {
            start,
            stop,
            fps,
            timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1. / (fps as f32)),
            bevy::time::TimerMode::Once,
        )
    }

    fn reset_timer(&mut self) {
        self.timer = AnimationConfig::timer_from_fps(self.fps);
    }
}

pub(super) fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut AnimationConfig)>,
) {
    let delta = time.delta();

    for (mut sp, mut cfg) in &mut query {
        cfg.timer.tick(delta);

        if cfg.timer.just_finished() {
            let Some(atlas) = &mut sp.texture_atlas else {
                continue;
            };

            if atlas.index == cfg.stop {
                atlas.index = cfg.start;
            } else {
                atlas.index += 1;
            }

            cfg.reset_timer();
        }
    }
}
