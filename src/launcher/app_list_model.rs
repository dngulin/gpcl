use super::QmlLauncher;

use strum::{EnumIter, FromRepr, IntoStaticStr};

#[derive(Clone, Copy, FromRepr, IntoStaticStr, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[repr(i32)]
enum FieldId {
    Name = 1,
    Icon,
}

impl From<FieldId> for i32 {
    fn from(value: FieldId) -> Self {
        value as i32
    }
}

impl QmlLauncher {
    fn get_item_field(&self, index: usize, role: i32) -> Option<()> {
        let item = self.launcher.items.get(index)?;
        let field_id = FieldId::from_repr(role)?;

        /*        let value = match field_id {
            FieldId::Name => QString::from(item.name.as_str()).to_qvariant(),
            FieldId::Icon => QString::from(item.icon.as_str()).to_qvariant(),
        };*/

        Some(())
    }
}
