use crate::GamepadModel;
use gilrs::{Gamepad, GamepadId, PowerInfo};
use std::time::Instant;

#[derive(Clone)]
pub struct GamepadItem {
    pub id: GamepadId,
    update_time: Instant,
    name: String,
    status: &'static str,
    charge: i32,
}

impl GamepadItem {
    pub fn get_seconds_since_last_update(&self) -> f32 {
        self.update_time.elapsed().as_secs_f32()
    }

    pub fn reset_update_time(&mut self) {
        self.update_time = Instant::now();
    }
}

impl<'a> From<Gamepad<'a>> for GamepadItem {
    fn from(value: Gamepad) -> Self {
        let (status, charge) = convert_power_info(value.power_info());
        Self {
            id: value.id(),
            update_time: Instant::now(),
            name: value.name().into(),
            status,
            charge,
        }
    }
}

pub fn convert_power_info(power_info: PowerInfo) -> (&'static str, i32) {
    match power_info {
        PowerInfo::Unknown | PowerInfo::Wired => ("Wired", 100),
        PowerInfo::Discharging(charge) => ("Discharging", charge as i32),
        PowerInfo::Charging(charge) => ("Charging", charge as i32),
        PowerInfo::Charged => ("Charging", 100),
    }
}

impl From<GamepadItem> for GamepadModel {
    fn from(value: GamepadItem) -> Self {
        Self {
            charge: value.charge,
            name: value.name.into(),
            status: value.status.into(),
        }
    }
}
