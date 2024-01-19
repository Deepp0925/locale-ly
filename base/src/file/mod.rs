pub mod item;

use std::{collections::hash_map::IterMut, fs::File, io::BufReader, path::Path};

use errors::{Errors, ErrorsResult};
use lingual::{Lang, Translator};
use serde_json::from_reader as json_from_reader;
use serde_yaml::{from_reader as yaml_from_reader, Mapping};

use crate::{file::item::JsonMap, pattern::RegexPattern};

use item::{IntoObjectType, ObjectItem, ObjectType, TranslateObjectType};

pub struct Object {
    pub items: ObjectType,
}

impl Object {
    fn open_file(path: impl AsRef<Path>) -> ErrorsResult<BufReader<File>> {
        let file = File::open(path.as_ref()).map_err(|err| Errors::OpenFile(err.to_string()))?;
        Ok(BufReader::new(file))
    }

    /// Opens a file and returns a hashmap of the file contents
    pub fn open_json(path: impl AsRef<Path>) -> ErrorsResult<Self> {
        let reader = Self::open_file(path)?;
        let data: JsonMap =
            json_from_reader(reader).map_err(|err| Errors::InvalidJson(err.to_string()))?;

        Ok(Self {
            items: data.into_object_type()?,
        })
    }

    /// Opens a file and returns a hashmap of the file contents
    pub fn open_yaml(path: impl AsRef<Path>) -> ErrorsResult<Self> {
        let reader = Self::open_file(path)?;
        let data: Mapping =
            yaml_from_reader(reader).map_err(|err| Errors::InvalidYaml(err.to_string()))?;

        Ok(Self {
            items: data.into_object_type()?,
        })
    }

    /// This iwll translate all the items in the object
    /// into a new object for the given language
    pub async fn translate_items(
        &mut self,
        regex: Option<RegexPattern>,
        translator: &Translator,
        src_lang: &Lang,
        lang: &Lang,
    ) -> ErrorsResult<ObjectType> {
        self.items
            .translate_items(regex, translator, src_lang, lang)
            .await
    }
}

impl<'a> Iterator for &'a Object {
    type Item = (&'a String, &'a ObjectItem);

    fn next(&mut self) -> Option<Self::Item> {
        self.items.iter().next()
    }
}

impl<'a> IntoIterator for &'a mut Object {
    type Item = (&'a String, &'a mut ObjectItem);
    type IntoIter = IterMut<'a, String, ObjectItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}
