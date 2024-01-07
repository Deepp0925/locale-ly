use crate::{parse::item::MappedObject, path::ParsedFileType};
use serde::Serialize;
use serde_json::{Map, Value as JsonValue};
use serde_yaml::Mapping;

pub struct JsonMap {
    pub map: Map<String, JsonValue>,
    pub len: usize,
}

impl Serialize for JsonMap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.map.serialize(serializer)
    }
}

impl JsonMap {
    pub fn is_full(&self) -> bool {
        self.len == self.map.len()
    }
}

pub enum ObjectType {
    JsonObject(JsonMap),
    YamlMapping(Mapping),
}

impl Serialize for ObjectType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::JsonObject(map) => map.serialize(serializer),
            Self::YamlMapping(map) => map.serialize(serializer),
        }
    }
}

impl ObjectType {
    pub fn from_file_type(file_type: ParsedFileType, items: usize) -> Self {
        match file_type {
            ParsedFileType::Json => {
                let map = Map::with_capacity(items);
                Self::JsonObject(JsonMap { map, len: items })
            }
            ParsedFileType::Yaml => {
                let map = Mapping::with_capacity(items);
                Self::YamlMapping(map)
            }
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        match self {
            Self::JsonObject(map) => {
                map.map.insert(key, value.into());
            }
            Self::YamlMapping(map) => {
                map.insert(key.into(), value.into());
            }
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            Self::JsonObject(map) => map.is_full(),
            Self::YamlMapping(map) => map.capacity() == map.len(),
        }
    }
}
