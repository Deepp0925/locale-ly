use std::collections::HashMap;

use errors::{Errors, ErrorsResult};
use lingual::{Lang, Translator};
use serde::{ser::SerializeSeq, Serialize};
use serde_json::{Map, Value as JsonValue};
use serde_yaml::{Mapping, Value as YamlValue};

use super::interpolated::Interpolated;
use crate::{pattern::RegexPattern, serializers::serialze_object};

pub type ObjectType = HashMap<String, ObjectItem>;

pub trait IntoObjectType {
    fn into_object_type(self) -> ErrorsResult<ObjectType>;
}

impl IntoObjectType for JsonMap {
    fn into_object_type(self) -> ErrorsResult<ObjectType> {
        let mut items = HashMap::with_capacity(self.len());

        for (key, value) in self {
            items.insert(key, value.into());
        }

        Ok(items)
    }
}

impl IntoObjectType for Mapping {
    fn into_object_type(self) -> ErrorsResult<ObjectType> {
        let mut items = HashMap::with_capacity(self.len());

        for (key, value) in self {
            if let YamlValue::String(key) = key {
                items.insert(key, value.into());
            } else {
                return Err(Errors::ExpectedString("Keys must be strings".to_string()));
            }
        }

        Ok(items)
    }
}

pub trait TranslateObjectType {
    async fn translate_items(
        &mut self,
        regex: Option<RegexPattern>,
        translator: &Translator,
        src_lang: &Lang,
        lang: &Lang,
    ) -> ErrorsResult<ObjectType>;
}

impl TranslateObjectType for ObjectType {
    async fn translate_items(
        &mut self,
        regex: Option<RegexPattern>,
        translator: &Translator,
        src_lang: &Lang,
        lang: &Lang,
    ) -> ErrorsResult<ObjectType> {
        let mut map = HashMap::with_capacity(self.len());
        for (k, v) in self {
            map.insert(
                k.clone(),
                v.translate(regex, translator, src_lang, lang).await?,
            );
        }

        Ok(map)
    }
}

#[derive(Debug)]
pub enum StringType {
    Regular(String),
    Interpolated(Interpolated),
}

impl StringType {
    /// Translates the string if the string is interpolated
    pub async fn translate(
        &mut self,
        regex: Option<RegexPattern>,
        translator: &Translator,
        src_lang: &Lang,
        lang: &Lang,
    ) -> ErrorsResult<String> {
        unsafe {
            let result = std::ptr::read(self);

            let ty = if let StringType::Regular(s) = result {
                let n = Interpolated::from_string(s, regex);
                StringType::Interpolated(n)
            } else {
                result
            };

            std::ptr::write(self, ty);
        };

        match self {
            // unreachable because of the match above
            StringType::Regular(_) => unreachable!(),
            StringType::Interpolated(s) => s.translate(translator, src_lang, lang).await,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            StringType::Regular(s) => s,
            StringType::Interpolated(s) => &s.txt,
        }
    }
}

impl From<StringType> for String {
    fn from(s: StringType) -> Self {
        match s {
            StringType::Regular(s) => s,
            StringType::Interpolated(s) => s.txt,
        }
    }
}

pub type JsonMap = Map<String, JsonValue>;
#[derive(Debug)]
pub enum ObjectItem {
    String(StringType),
    Array(Vec<StringType>),
    Object(ObjectType),
}

impl ObjectItem {
    #[async_recursion::async_recursion]
    pub async fn translate(
        &mut self,
        regex: Option<RegexPattern>,
        translator: &Translator,
        src: &Lang,
        lang: &Lang,
    ) -> ErrorsResult<Self> {
        match self {
            ObjectItem::String(s) => Ok(Self::String(StringType::Regular(
                s.translate(regex, translator, src, lang).await?,
            ))),
            ObjectItem::Array(arr) => {
                let mut items = Vec::with_capacity(arr.len());

                for item in arr {
                    items.push(StringType::Regular(
                        item.translate(regex, translator, src, lang).await?,
                    ));
                }

                Ok(Self::Array(items))
            }
            ObjectItem::Object(map) => map
                .translate_items(regex, translator, src, lang)
                .await
                .map(Self::Object),
        }
    }
}

impl Serialize for ObjectItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ObjectItem::String(s) => serializer.serialize_str(s.as_str()),
            ObjectItem::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr {
                    seq.serialize_element(item.as_str())?;
                }

                seq.end()
            }
            ObjectItem::Object(map) => serialze_object(serializer, map),
        }
    }
}

impl From<ObjectItem> for JsonValue {
    fn from(item: ObjectItem) -> Self {
        match item {
            ObjectItem::String(s) => JsonValue::String(s.into()),
            ObjectItem::Array(arr) => {
                let mut items = Vec::with_capacity(arr.len());

                for item in arr {
                    items.push(JsonValue::String(item.into()));
                }

                JsonValue::Array(items)
            }
            ObjectItem::Object(map) => {
                let mut items = JsonMap::with_capacity(map.len());

                for (key, value) in map {
                    items.insert(key, value.into());
                }

                JsonValue::Object(items)
            }
        }
    }
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
            JsonValue::String(s) => ObjectItem::String(StringType::Regular(s)),
            JsonValue::Array(arr) => {
                let mut items = Vec::with_capacity(arr.len());

                for item in arr {
                    if let JsonValue::String(s) = item {
                        items.push(StringType::Regular(s));
                    } else {
                        panic!("Array items must be strings")
                    }
                }

                ObjectItem::Array(items)
            }
            JsonValue::Object(map) => map.into(),
            JsonValue::Number(num) => ObjectItem::String(StringType::Regular(num.to_string())),
            _ => panic!("Unsupported JSON value"),
        }
    }
}

impl From<ObjectItem> for YamlValue {
    fn from(item: ObjectItem) -> Self {
        match item {
            ObjectItem::String(s) => YamlValue::String(s.into()),
            ObjectItem::Array(arr) => {
                let mut items = Vec::with_capacity(arr.len());

                for item in arr {
                    items.push(YamlValue::String(item.into()));
                }

                YamlValue::Sequence(items)
            }
            ObjectItem::Object(map) => {
                let mut items = Mapping::with_capacity(map.len());

                for (key, value) in map {
                    items.insert(key.into(), value.into());
                }

                YamlValue::Mapping(items)
            }
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
            YamlValue::String(s) => ObjectItem::String(StringType::Regular(s)),
            YamlValue::Sequence(arr) => {
                let mut items = Vec::with_capacity(arr.len());

                for item in arr {
                    if let YamlValue::String(s) = item {
                        items.push(StringType::Regular(s));
                    } else {
                        panic!("Array items must be strings")
                    }
                }

                ObjectItem::Array(items)
            }
            YamlValue::Mapping(map) => map.into(),
            YamlValue::Number(num) => ObjectItem::String(StringType::Regular(num.to_string())),
            _ => panic!("Unsupported YAML value"),
        }
    }
}
