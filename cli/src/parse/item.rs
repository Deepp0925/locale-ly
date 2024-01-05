use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::collections::{hash_map::IterMut, HashMap};
/// This module will read the file and generate a hashmap in the following format:
/// Key: Value
/// where `key` is a string and the `value` is a can either be a string or a hashmap
/// of strings, At the moment only a depth of 1 is supported for hashmaps
/// and no support for arrays is provided at the moment. Arrays might be supported later on
/// but for now it is regarded as a low priority and not widely used.
#[derive(Debug, Clone)]
pub enum ItemType {
    String(String),
    // Array(Vec<String>),
    Object(HashMap<String, String>),
}

impl<'a> IntoIterator for &'a mut ItemType {
    type Item = (&'a String, &'a mut String);
    type IntoIter = IterMut<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ItemType::String(_) => unimplemented!(),
            ItemType::Object(map) => map.iter_mut(),
        }
    }
}

impl From<JsonValue> for ItemType {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::String(string) => Self::String(string),
            JsonValue::Object(object) => {
                let mut map = HashMap::with_capacity(object.len());
                for (key, value) in object {
                    let val = match value {
                        JsonValue::String(string) => string,
                        JsonValue::Number(number) => number.to_string(),
                        _ => todo!(),
                    };

                    map.insert(key, val);
                }
                Self::Object(map)
            }
            _ => todo!(),
        }
    }
}

impl From<YamlValue> for ItemType {
    fn from(value: YamlValue) -> Self {
        match value {
            YamlValue::String(string) => Self::String(string),
            YamlValue::Mapping(mapping) => {
                let mut items = HashMap::with_capacity(mapping.len());
                for (key, value) in mapping {
                    let key = match key {
                        YamlValue::String(string) => string,
                        _ => todo!(),
                    };

                    let value = match value {
                        YamlValue::String(string) => string,
                        YamlValue::Number(number) => number.to_string(),
                        _ => panic!("{}", t!("expected_string", key = key)),
                    };

                    items.insert(key, value);
                }
                Self::Object(items)
            }
            _ => todo!(),
        }
    }
}
