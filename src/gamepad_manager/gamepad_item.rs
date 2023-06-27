use gilrs::GamepadId;
use std::time::Instant;

#[derive(Clone)]
pub struct GamepadItem {
    pub id: GamepadId,
    update_time: Instant,
}

impl GamepadItem {
    pub fn new(id: GamepadId) -> Self {
        Self {
            id,
            update_time: Instant::now(),
        }
    }

    pub fn get_seconds_since_last_update(&self) -> f32 {
        self.update_time.elapsed().as_secs_f32()
    }

    pub fn reset_update_time(&mut self) {
        self.update_time = Instant::now();
    }
}
