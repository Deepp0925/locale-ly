use serde_json::{Map, Value};
use serde_yaml::Mapping;

pub enum ObjectType {
    Json(Map<String, Value>),
    Yaml(Mapping),
}
