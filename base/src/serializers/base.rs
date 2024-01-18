use serde_json::Value as JsonValue;
use serde_yaml::{Mapping, Value as YamlValue};

use crate::file::item::JsonMap;

pub trait Serializable<T> {
    fn add(&mut self, key: String, value: T);
}

impl<T: Into<YamlValue>> Serializable<T> for Mapping {
    fn add(&mut self, key: String, value: T) {
        self.insert(key.into(), value.into());
    }
}

impl<T: Into<JsonValue>> Serializable<T> for JsonMap {
    fn add(&mut self, key: String, value: T) {
        self.insert(key, value.into());
    }
}
