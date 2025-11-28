use crate::{GamepadModel, GamepadStatus};
use gilrs::{Gamepad, GamepadId, PowerInfo};
use std::time::Instant;

#[derive(Clone)]
pub struct GamepadTrackingState {
    pub gamepad_id: GamepadId,
    update_time: Instant,
}

impl GamepadTrackingState {
    pub fn new(gamepad_id: GamepadId) -> Self {
        Self {
            gamepad_id,
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

pub fn create_model_and_tracking_state(value: Gamepad) -> (GamepadModel, GamepadTrackingState) {
    (value.into(), GamepadTrackingState::new(value.id()))
}

fn convert_power_info(power_info: PowerInfo) -> (GamepadStatus, i32) {
    match power_info {
        PowerInfo::Unknown | PowerInfo::Wired => (GamepadStatus::Wired, 100),
        PowerInfo::Discharging(charge) => (GamepadStatus::Discharging, charge as i32),
        PowerInfo::Charging(charge) => (GamepadStatus::Charging, charge as i32),
        PowerInfo::Charged => (GamepadStatus::Charging, 100),
    }
}

impl<'a> From<Gamepad<'a>> for GamepadModel {
    fn from(gamepad: Gamepad<'a>) -> GamepadModel {
        let name = gamepad.name().into();
        let (status, charge) = convert_power_info(gamepad.power_info());

        GamepadModel {
            name,
            status,
            charge,
        }
    }
}

pub trait UpdatePowerInfo {
    fn update_power_info(&mut self, power_info: PowerInfo) -> bool;
}

impl UpdatePowerInfo for GamepadModel {
    fn update_power_info(&mut self, power_info: PowerInfo) -> bool {
        let (status, charge) = convert_power_info(power_info);

        if self.status != status || self.charge != charge {
            self.status = status;
            self.charge = charge;
            return true;
        }

        false
    }
}
