use serde_json::{map::Iter as JsonIter, Map, Value as JsonValue};
use serde_yaml::{mapping::Iter as YamlIter, Mapping, Value as YamlValue};

pub type MappedObject = Map<String, JsonValue>;
pub type JsonMapItem<'a> = (&'a String, &'a JsonValue);
pub type YamlMapItem<'a> = (&'a YamlValue, &'a YamlValue);

pub enum ObjectType {
    Json(MappedObject),
    Yaml(Mapping),
}

impl From<Mapping> for ObjectType {
    fn from(mapping: Mapping) -> Self {
        Self::Yaml(mapping)
    }
}

impl From<MappedObject> for ObjectType {
    fn from(map: MappedObject) -> Self {
        Self::Json(map)
    }
}

impl<'a> IntoIterator for &'a ObjectType {
    type Item = ObjectIterItem<'a>;
    type IntoIter = ObjectIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ObjectType::Json(map) => ObjectIter::Json(map.iter()),
            ObjectType::Yaml(map) => ObjectIter::Yaml(map.iter()),
        }
    }
}

pub enum ObjectIter<'a> {
    Json(JsonIter<'a>),
    Yaml(YamlIter<'a>),
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = ObjectIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Json(iter) => iter.next().map(Into::into),
            Self::Yaml(iter) => iter.next().map(Into::into),
        }
    }
}

pub enum ObjectIterItem<'a> {
    Json(JsonMapItem<'a>),
    Yaml(YamlMapItem<'a>),
}

impl<'a> From<JsonMapItem<'a>> for ObjectIterItem<'a> {
    fn from(item: JsonMapItem<'a>) -> Self {
        Self::Json(item)
    }
}

impl<'a> From<YamlMapItem<'a>> for ObjectIterItem<'a> {
    fn from(item: YamlMapItem<'a>) -> Self {
        Self::Yaml(item)
    }
}
