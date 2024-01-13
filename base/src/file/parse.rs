use std::vec::IntoIter;

use lingual::Lang;

use super::object::ObjectType;

pub struct ParseFile {
    pub src: Lang,
    pub targets: IntoIter<Lang>,
    /// These are all the items in the file.
    /// that should be used to generate the other locale files.
    pub items: ObjectType,
}

impl ParseFile {
    pub fn translate_next_lang(&self) {
        todo!()
    }

    pub fn translate_all_langs(&self) {
        todo!()
    }
}

/// attempts to parse the file into the builder
pub struct ParseFileBuilder {
    pub src: Option<Lang>,
    pub targets: Option<Vec<Lang>>,
    pub items: ObjectType,
}
