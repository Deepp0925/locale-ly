use serde_json::{Map, Value as JsonValue};
use serde_yaml::Mapping;

pub enum ObjectType {
    Json(Map<String, JsonValue>),
    Yaml(Mapping),
}
