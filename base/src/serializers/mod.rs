pub mod json;
pub mod yaml;
use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use errors::{Errors, ErrorsResult};
use lingual::Lang;
use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};
use serde_json::Serializer as JsonSerializer;
use serde_yaml::Serializer as YamlSerializer;

use crate::{
    parser::item::ObjectType,
    path::{FileType, LocalePaths},
};

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
pub type AllWritersType = HashMap<Lang, Vec<WriteSerializer>>;

pub enum WriteSerializer {
    Json(JsonSerializer<WriterType>),
    Yaml(YamlSerializer<WriterType>),
}

impl WriteSerializer {
    pub fn from_file_type(file_type: &FileType, file: File) -> Self {
        let writer = BufWriter::new(file);

        match file_type {
            FileType::Json => Self::Json(JsonSerializer::new(writer)),
            FileType::Yaml => Self::Yaml(YamlSerializer::new(writer)),
        }
    }

    fn serialize(&mut self, object: &ObjectType) -> ErrorsResult<()> {
        match self {
            WriteSerializer::Json(w) => {
                serialze_object(w, object).map_err(|err| Errors::Serialize(err.to_string()))?
            }
            WriteSerializer::Yaml(w) => {
                serialze_object(w, object).map_err(|err| Errors::Serialize(err.to_string()))?
            }
        }

        Ok(())
    }
}

pub struct Writers {
    /// There can be multiple writers for each language
    /// depending upon the scale of the project.
    /// The developers might want yaml, json, and strings files
    /// for each language.
    writers: AllWritersType,
}

impl Writers {
    /// writes all the parsed values to the files
    /// This will serialize into each file formats and write them to the files.
    /// This function assumes that the lang is already present in the hashmap.
    /// It simply returns an Ok(()) if the lang is not present.
    pub fn write_all(&mut self, lang: &Lang, object: &ObjectType) -> ErrorsResult<()> {
        if let Some(writers) = self.writers.get_mut(lang) {
            for writer in writers.iter_mut() {
                writer.serialize(object)?;
            }
        }

        Ok(())
    }

    /// Creates a new instance of the writers.
    /// # Arguments
    /// * `langs` - The languages for which the files are to be created.
    /// * `types` - The file types for which the files are to be created. (json, yaml, strings)
    /// * `src_file` - The source file from which the files are to be created.
    pub async fn from_file_types(
        types: &[FileType],
        langs: &[Lang],
        src_file: impl AsRef<Path>,
    ) -> ErrorsResult<Self> {
        let files = src_file.create_locale_files(langs, types).await?;
        Ok(Self { writers: files })
    }
}
