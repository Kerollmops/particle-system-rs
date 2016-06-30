use time::{Duration, PreciseTime};

#[derive(Clone, Copy)]
pub enum AnimationType {
    RandCube,
    Cube,
    RandSphere
}

pub struct Animation {
    anim_type: AnimationType,
    duration: Duration,
    start: PreciseTime,
    currently_in_animation: bool
}

impl Animation {
    pub fn now(anim_type: AnimationType, duration: Duration) -> Animation {
        Animation {
            anim_type: anim_type,
            duration: duration,
            start: PreciseTime::now(),
            currently_in_animation: true
        }
    }
}
