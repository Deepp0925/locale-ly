pub mod locales;
pub mod translation;
pub mod translations;

/// This module is solely responsible for handling file operations.
/// To be specific, these operations will be handled by sled(embedded database) internally.
/// This module will be used to parse the translations and locales from the files and
/// generate the file output file for the user at the given path.

const TRANSLATIONS_TABLE: &str = "translations";
const LOCALES_TABLE: &str = "locales";
/// This will be used later on for other parts but right now it's not needed
// const SETTINGS_TABLE: &str = "settings";

pub struct FileDb {
    pub db: sled::Db,
    pub locales: locales::Locales,
    pub translations: sled::Tree, // TODO change this to transaltions struct
}

impl FileDb {
    /// writes the locales to the locales key in the database
    fn write_locales(&self) -> Result<(), sled::Error> {
        // let locales = self.locales.as_b
        self.db.insert(LOCALES_TABLE, self.locales.as_bytes())?;

        Ok(())
    }
}
