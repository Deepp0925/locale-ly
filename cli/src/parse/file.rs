use serde_json::{from_reader as json_from_reader, Map, Value as JsonValue};
use serde_yaml::{from_reader as yaml_from_reader, Mapping, Value as YamlValue};
use std::{
    collections::{
        hash_map::{IntoIter, IterMut},
        HashMap,
    },
    fs::File,
    io::BufReader,
    path::Path,
};

use super::item::ItemType;

pub type MappedItems = HashMap<String, ItemType>;
#[derive(Debug, Clone)]
pub struct ParseFile {
    pub items: MappedItems,
}
pub type JsonMap = Map<String, JsonValue>;

impl From<Mapping> for ParseFile {
    fn from(mapping: Mapping) -> Self {
        let mut items = HashMap::with_capacity(mapping.len());
        for (key, value) in mapping {
            let key = match key {
                YamlValue::String(string) => string,
                _ => todo!(),
            };

            items.insert(key, value.into());
        }
        Self { items }
    }
}

impl From<JsonMap> for ParseFile {
    fn from(map: JsonMap) -> Self {
        let mut items = HashMap::with_capacity(map.len());
        for (key, value) in map {
            items.insert(key, value.into());
        }
        Self { items }
    }
}

impl<'a> IntoIterator for &'a mut ParseFile {
    type Item = (&'a String, &'a mut ItemType);
    type IntoIter = IterMut<'a, String, ItemType>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl IntoIterator for ParseFile {
    type Item = (String, ItemType);
    type IntoIter = IntoIter<String, ItemType>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl ParseFile {
    fn open_file(path: impl AsRef<Path>) -> BufReader<File> {
        let file =
            File::open(path.as_ref()).expect(&t!("file_not_found", path = path.as_ref().display()));
        BufReader::new(file)
    }

    /// Opens a file and returns a hashmap of the file contents
    pub fn open_json(path: impl AsRef<Path>) -> Self {
        let reader = Self::open_file(path);
        let data: JsonMap = json_from_reader(reader).expect(&t!("file_expected_key_value"));

        Self::from(data)
    }

    /// Opens a file and returns a hashmap of the file contents
    pub fn open_yaml(path: impl AsRef<Path>) -> Self {
        let reader = Self::open_file(path);
        let data: Mapping = yaml_from_reader(reader).expect(&t!("file_expected_key_value"));
        Self::from(data)
    }
}

mod test {

    #[test]
    fn tessst() {
        use super::ParseFile;
        let file = ParseFile::open_yaml("../assets/locales/en.yml");

        println!("{:#?}", file);
    }
}
