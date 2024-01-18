use std::{collections::HashMap, fs::File, hash::Hash, io::BufReader, path::Path};

use serde_json::{from_reader as json_from_reader, map::Iter as JsonIter, Map, Value as JsonValue};
use serde_yaml::{
    from_reader as yaml_from_reader, mapping::Iter as YamlIter, Mapping, Value as YamlValue,
};

use super::item::ObjectType;

pub struct Object {
    pub items: ObjectType,
}

// impl Object {
//     fn open_file(path: impl AsRef<Path>) -> BufReader<File> {
//         let file =
//             File::open(path.as_ref()).expect(&t!("file_not_found", path = path.as_ref().display()));
//         BufReader::new(file)
//     }

//     /// Opens a file and returns a hashmap of the file contents
//     pub fn open_json(path: impl AsRef<Path>) -> Self {
//         let reader = Self::open_file(path);
//         let data: JsonMap = json_from_reader(reader).expect(&t!("file_expected_key_value"));

//         todo!()
//         // Self {
//         //     items: ObjectItem::from(data),
//         // }
//     }

//     /// Opens a file and returns a hashmap of the file contents
//     pub fn open_yaml(path: impl AsRef<Path>) -> Self {
//         let reader = Self::open_file(path);
//         let data: Mapping = yaml_from_reader(reader).expect(&t!("file_expected_key_value"));
//         todo!()
//         // Self {
//         //     items: ObjectItem::from(data),
//         // }
//     }
// }
