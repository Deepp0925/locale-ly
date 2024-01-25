use crate::parser::item::ObjectType;
use serde_yaml::Mapping;

pub trait IntoYamlObject {
    fn into_yaml_object(self) -> Mapping;
}

impl IntoYamlObject for ObjectType {
    fn into_yaml_object(self) -> Mapping {
        let mut map = Mapping::with_capacity(self.len());

        for (key, value) in self {
            map.insert(key.into(), value.into());
        }

        map
    }
}
