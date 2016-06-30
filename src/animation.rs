use std::default::Default;
use time::{Duration, PreciseTime};
use particles::Particles;

#[derive(Clone, Copy)]
pub enum AnimationType {
    RandCube,
    Cube,
    RandSphere
}

impl Default for AnimationType {
    fn default() -> Self {
        AnimationType::RandCube
    }
}

pub struct Animation {
    anim_type: AnimationType,
    duration: Duration,
    start: PreciseTime,
    currently_in_animation: bool
}

impl Animation {
    pub fn new(duration: Duration) -> Animation {
        Animation {
            anim_type: AnimationType::default(),
            duration: duration,
            start: PreciseTime::now(),
            currently_in_animation: false
        }
    }

    pub fn init_now(&mut self, particles: &mut Particles) {
        match self.anim_type {
            AnimationType::Cube => particles.init_cube_animation(self.duration),
            AnimationType::RandCube => particles.init_rand_cube_animation(self.duration),
            AnimationType::RandSphere => particles.init_rand_sphere_animation(self.duration),
        };
        self.start = PreciseTime::now();
        self.currently_in_animation = true
    }

    pub fn set_animation(&mut self, anim_type: AnimationType) {
        self.anim_type = anim_type
    }

    pub fn currently_in_animation(&self) -> bool {
        self.currently_in_animation
    }

    pub fn update(&mut self, particles: &mut Particles) {
        if self.currently_in_animation() == true {
            let elaps_time_anim = self.start.to(PreciseTime::now());
            if elaps_time_anim <= self.duration {
                particles.update_animation(elaps_time_anim);
            }
            else {
                particles.update_animation(self.duration);
                self.currently_in_animation = false;
            }
        }
    }
}
