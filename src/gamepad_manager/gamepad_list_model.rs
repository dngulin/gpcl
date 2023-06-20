use super::QmlGamepadManager;

use gilrs::{Gamepad, GamepadId, PowerInfo};
use std::time::Instant;
use strum::{EnumIter, FromRepr, IntoStaticStr};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum QmlPowerStatus {
    Wired = 1,
    Discharging = 2,
    Charging = 3,
    Charged = 4,
}

pub struct Item {
    pub id: GamepadId,
    update_time: Instant,

    pub name: String,
    pub status: QmlPowerStatus,
    pub charge: i32,
}

impl Item {
    pub fn get_seconds_since_last_update(&self) -> f32 {
        self.update_time.elapsed().as_secs_f32()
    }

    pub fn reset_update_time(&mut self) {
        self.update_time = Instant::now();
    }
}

#[derive(Clone, Copy, FromRepr, IntoStaticStr, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[repr(i32)]
enum FieldId {
    Name = 1,
    Status,
    Charge,
}

impl From<FieldId> for i32 {
    fn from(value: FieldId) -> Self {
        value as i32
    }
}

impl QmlGamepadManager {
    fn get_item_field(&self, index: usize, role: i32) -> Option<()> {
        let item = self.gamepads.get(index)?;
        let field_id = FieldId::from_repr(role)?;

        /*        let value = match field_id {
                    FieldId::Name => QVariant::from(&item.name),
                    FieldId::Status => QVariant::from(item.status as i32),
                    FieldId::Charge => QVariant::from(item.charge),
                };
        */
        Some(())
    }
}

impl<'a> From<Gamepad<'a>> for Item {
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

pub fn convert_power_info(power_info: PowerInfo) -> (QmlPowerStatus, i32) {
    match power_info {
        PowerInfo::Unknown | PowerInfo::Wired => (QmlPowerStatus::Wired, 100),
        PowerInfo::Discharging(charge) => (QmlPowerStatus::Discharging, charge as i32),
        PowerInfo::Charging(charge) => (QmlPowerStatus::Charging, charge as i32),
        PowerInfo::Charged => (QmlPowerStatus::Charging, 100),
    }
}