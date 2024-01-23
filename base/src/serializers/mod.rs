pub mod json;
pub mod yaml;
use std::{collections::HashMap, fs::File, io::BufWriter};

use errors::{Errors, ErrorsResult};
use lingual::Lang;
use serde::ser::{SerializeMap, Serializer};
use serde_json::Serializer as JsonSerializer;
use serde_yaml::Serializer as YamlSerializer;

use crate::parser::item::ObjectType;

pub fn serialze_object<S: Serializer>(
    serializer: S,
    object: &ObjectType,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(object.len()))?;

    for (key, value) in object {
        map.serialize_entry(key, value)?;
    }

    map.end()
}

type WriterType = BufWriter<File>;

pub enum WriteSerializers {
    Json(JsonSerializer<WriterType>),
    Yaml(YamlSerializer<WriterType>),
}

pub struct Writers {
    /// There can be multiple writers for each language
    /// depending upon the scale of the project.
    /// The developers might want yaml, json, and strings files
    /// for each language.
    writers: HashMap<Lang, Vec<WriteSerializers>>,
}

impl Writers {
    /// Translates all the provided langs and writes them to the respective files
    fn write_all(&mut self, parsed_values: ObjectType) -> ErrorsResult<()> {
        // for (lang, writers) in self.writers.iter_mut() {
        //     let object =
        //     for writer in writers.iter_mut() {
        //         match writer {
        //             WriteSerializers::Json(w) => serialze_object(w, object),
        //             WriteSerializers::Yaml(w) => serialze_object(w, object),
        //         }
        //     }
        // }

        // let writer = self.writers.get_mut(lang).ok_or(Errors::Serialize(""))?;
        // for serializer in writer.iter_mut() {
        //     match serializer {
        //         WriteSerializers::Json(w) => {
        //             serialze_object(w, object).map_err(|err| Errors::Serialize(err.to_string()))?;
        //         }
        //         WriteSerializers::Yaml(w) => {
        //             serialze_object(w, object).map_err(|err| Errors::Serialize(err.to_string()))?;
        //         }
        //     }
        // }

        Ok(())
    }
}
