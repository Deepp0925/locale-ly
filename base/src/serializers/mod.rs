pub mod json;
pub mod yaml;
use serde::ser::{SerializeMap, Serializer};

use crate::file::item::ObjectType;

pub fn serialze_object<S: Serializer>(
    serializer: S,
    object: &ObjectType,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(object.len()))?;

    for (key, value) in object {
        map.serialize_entry(key, value)?;
    }

    map.end()
}
