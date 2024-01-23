use crate::parser::item::ObjectType;
use serde_json::{Map as JsonMap, Value as JsonValue};
pub trait IntoJsonObject {
    fn into_json_object(self) -> JsonMap<String, JsonValue>;
}

impl IntoJsonObject for ObjectType {
    fn into_json_object(self) -> JsonMap<String, JsonValue> {
        let mut map = JsonMap::with_capacity(self.len());

        for (key, value) in self {
            map.insert(key, value.into());
        }

        map
    }
}
