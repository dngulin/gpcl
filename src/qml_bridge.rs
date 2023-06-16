use qttypes::QByteArray;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub fn role_names<TEnum>() -> HashMap<i32, QByteArray>
where
    TEnum: Copy + IntoEnumIterator + Into<&'static str> + Into<i32>,
{
    let map = TEnum::iter().map(|field_id| {
        let field_name: &str = field_id.into();
        let key: i32 = field_id.into();
        let val: QByteArray = field_name.into();
        (key, val)
    });

    HashMap::from_iter(map)
}
