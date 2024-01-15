use std::{collections::HashMap, hash::Hash};

use serde_json::{map::Iter as JsonIter, Map, Value as JsonValue};
use serde_yaml::{mapping::Iter as YamlIter, Mapping, Value as YamlValue};

pub type MappedObject = Map<String, JsonValue>;
pub type JsonMapItem<'a> = (&'a String, &'a JsonValue);
pub type YamlMapItem<'a> = (&'a YamlValue, &'a YamlValue);

pub enum ObjectType {
    String(String),
    Array(Vec<String>),
    Object(HashMap<String, ObjectType>),
}

pub struct Object {
    pub items: MappedObject,
}
