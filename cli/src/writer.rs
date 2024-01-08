use errors::{Errors, ErrorsResult};
use lingual::Lang;
use serde::Serialize;
// use serde::ser::SerializeMap;
use serde_json::{Map as JsonMap, Serializer as JsonSerializer};
use serde_yaml::{Mapping, Serializer as YamlSerializer};

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::{object::ObjectType, path::ParsedFileType};

enum SerializerType {
    Json(JsonSerializer<File>),
    Yaml(YamlSerializer<File>),
}

impl From<&SerializerType> for ParsedFileType {
    fn from(serializer: &SerializerType) -> Self {
        match serializer {
            SerializerType::Json(_) => Self::Json,
            SerializerType::Yaml(_) => Self::Yaml,
        }
    }
}

struct WriterInnerType {
    w: SerializerType,
    objects: Option<ObjectType>,
}

impl WriterInnerType {
    fn serialize(&mut self, s: impl Serialize) -> ErrorsResult<()> {
        match &mut self.w {
            SerializerType::Json(w) => {
                s.serialize(w)
                    .map_err(|err| Errors::Serialize(err.to_string()))?;
            }
            SerializerType::Yaml(w) => {
                s.serialize(w)
                    .map_err(|err| Errors::Serialize(err.to_string()))?;
            }
        }

        Ok(())
    }

    fn force_write_objects(&mut self) -> ErrorsResult<()> {
        let objects = if let Some(obj) = self.objects.take() {
            obj
        } else {
            return Ok(());
        };

        self.serialize(objects)
    }

    fn write_object_on_full(
        &mut self,
        key: String,
        value: String,
        file_type: ParsedFileType,
    ) -> ErrorsResult<()> {
        if let Some(object) = &self.objects {
            if object.is_full() {
                self.force_write_objects()?;
            }
        }

        // create a object type that can hold 5 items
        // after which it will be serialized and written to the file
        self.objects
            .get_or_insert(ObjectType::from_file_type(file_type, 5))
            .insert(key, value);

        Ok(())
    }

    fn push(&mut self, key: String, value: String) -> ErrorsResult<()> {
        let file_type = ParsedFileType::from(&self.w);
        self.write_object_on_full(key, value, file_type)?;
        Ok(())
    }

    fn push_object(&mut self, key: String, obj: ObjectType) -> ErrorsResult<()> {
        self.force_write_objects()?;
        match obj {
            ObjectType::JsonObject(json_map) => {
                let mut new_map = JsonMap::with_capacity(1);
                new_map.insert(key, json_map.map.into());
                self.serialize(new_map)?;
            }
            ObjectType::YamlMapping(mapping) => {
                let mut new_map = Mapping::with_capacity(1);
                new_map.insert(key.into(), mapping.into());
                self.serialize(new_map)?;
            }
        }
        Ok(())
    }

    fn into_inner_writer(mut self) -> ErrorsResult<File> {
        self.force_write_objects()?;
        match self.w {
            SerializerType::Json(w) => Ok(w.into_inner()),
            SerializerType::Yaml(w) => w
                .into_inner()
                .map_err(|err| Errors::Serialize(err.to_string())),
        }
    }

    /// removes the `---` lines from the file
    /// this is only needed for yaml files
    fn finish(self) -> ErrorsResult<()> {
        let file_type = ParsedFileType::from(&self.w);
        let file = self.into_inner_writer()?;

        if file_type == ParsedFileType::Json {
            return Ok(());
        }

        //TODO todo!("remove the --- lines from the file");

        Ok(())
    }
}

pub struct Writers {
    writers: HashMap<Lang, WriterInnerType>,
}

impl Writers {
    pub fn from_file_map(file_map: HashMap<Lang, File>, file_type: ParsedFileType) -> Self {
        let writers = file_map
            .into_iter()
            .map(|(lang, w)| {
                let w = match file_type {
                    ParsedFileType::Json => SerializerType::Json(JsonSerializer::new(w)),
                    ParsedFileType::Yaml => SerializerType::Yaml(YamlSerializer::new(w)),
                };

                (lang, WriterInnerType { w, objects: None })
            })
            .collect();

        Self { writers }
    }

    pub fn push_object(&mut self, lang: &Lang, key: String, obj: ObjectType) -> ErrorsResult<()> {
        // safe to unwrap because we know that the lang is in the map
        let writer = self.writers.get_mut(lang).unwrap();

        writer.push_object(key, obj)
    }

    pub fn push(&mut self, lang: &Lang, key: String, value: String) -> ErrorsResult<()> {
        // safe to unwrap because we know that the lang is in the map
        let writer = self.writers.get_mut(lang).unwrap();
        writer.push(key, value)
    }

    pub fn finish(self) -> ErrorsResult<()> {
        // yaml files add a `---` between each sections
        // so this is a hack to remove all the `---\n`s
        for (_, writer) in self.writers {
            writer.finish()?;
        }

        Ok(())
    }
}
