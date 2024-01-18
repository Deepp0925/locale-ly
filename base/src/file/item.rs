use std::collections::HashMap;

use serde_json::{Map, Value as JsonValue};
use serde_yaml::{Mapping, Value as YamlValue};

pub type ObjectType = HashMap<String, ObjectItem>;
pub type JsonMap = Map<String, JsonValue>;
pub enum ObjectItem {
    String(String),
    Array(Vec<String>),
    Object(ObjectType),
}

impl From<JsonMap> for ObjectItem {
    fn from(map: JsonMap) -> Self {
        let mut items = HashMap::with_capacity(map.len());

        for (key, value) in map {
            items.insert(key, value.into());
        }

        ObjectItem::Object(items)
    }
}

impl From<JsonValue> for ObjectItem {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::String(s) => ObjectItem::String(s),
            JsonValue::Array(arr) => {
                let mut items = Vec::with_capacity(arr.len());

                for item in arr {
                    if let JsonValue::String(s) = item {
                        items.push(s);
                    } else {
                        panic!("Array items must be strings")
                    }
                }

                ObjectItem::Array(items)
            }
            JsonValue::Object(map) => map.into(),
            JsonValue::Number(num) => ObjectItem::String(num.to_string()),
            _ => panic!("Unsupported JSON value"),
        }
    }
}

impl From<Mapping> for ObjectItem {
    fn from(value: Mapping) -> Self {
        let mut items = HashMap::with_capacity(value.len());

        for (key, value) in value {
            if let YamlValue::String(key) = key {
                items.insert(key, value.into());
            } else {
                panic!("Keys must be strings")
            }
        }

        ObjectItem::Object(items)
    }
}

impl From<YamlValue> for ObjectItem {
    fn from(value: YamlValue) -> Self {
        match value {
            YamlValue::String(s) => ObjectItem::String(s),
            YamlValue::Sequence(arr) => {
                let mut items = Vec::with_capacity(arr.len());

                for item in arr {
                    if let YamlValue::String(s) = item {
                        items.push(s);
                    } else {
                        panic!("Array items must be strings")
                    }
                }

                ObjectItem::Array(items)
            }
            YamlValue::Mapping(map) => map.into(),
            YamlValue::Number(num) => ObjectItem::String(num.to_string()),
            _ => panic!("Unsupported YAML value"),
        }
    }
}
