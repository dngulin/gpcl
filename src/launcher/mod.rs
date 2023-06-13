use qmetaobject::prelude::*;
use qmetaobject::USER_ROLE;
use std::collections::HashMap;
use strum::{EnumIter, FromRepr, IntoEnumIterator, IntoStaticStr};

#[derive(QObject, Default)]
pub struct QmlLauncher {
    base: qt_base_class!(trait QAbstractListModel),

    init: qt_method!(fn(&mut self) -> bool),
    exec_item: qt_method!(fn(&self, idx: usize) -> bool),
    has_running_item: qt_method!(fn(&mut self) -> bool),
}

impl QmlLauncher {
    fn init(&mut self) -> bool {
        todo!()
    }

    fn exec_item(&mut self, idx: usize) -> bool {
        todo!()
    }

    fn has_running_item(&mut self) -> bool {
        todo!()
    }
}

#[derive(Clone, Copy, FromRepr, IntoStaticStr, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[repr(i32)]
enum ItemFieldRole {
    Name = USER_ROLE + 1,
    Icon,
}

impl QAbstractListModel for QmlLauncher {
    fn row_count(&self) -> i32 {
        todo!()
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        todo!()
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let map = ItemFieldRole::iter().map(|role| {
            let str_val: &'static str = role.into();
            let val: QByteArray = str_val.into();
            (role as i32, val)
        });

        HashMap::from_iter(map)
    }
}
