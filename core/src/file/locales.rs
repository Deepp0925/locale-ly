use lingual::AccurateLang;
use serde::{Deserialize, Serialize};

/// Information related to locales will be stored in this struct.
/// This struct instructs the destination of locales and the source of locales.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Locales {
    pub src: AccurateLang,
    pub locales: Vec<AccurateLang>,
}

impl Locales {
    /// creates a new instance of locales
    pub fn new(src: AccurateLang) -> Self {
        Self {
            src,
            locales: vec![],
        }
    }

    /// adds a locale to the locales
    pub fn add_locale(&mut self, locale: AccurateLang) {
        self.locales.push(locale);
    }

    /// removes a locale from the locales
    pub fn remove_locale(&mut self, locale: AccurateLang) {
        self.locales.retain(|l| l != &locale);
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
