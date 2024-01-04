pub mod file;
pub mod parsers;
pub mod pattern;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AllTranslationsInner {
    pub locales: Vec<String>,
    pub translations: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllTranslations {
    pub inner: Arc<RwLock<AllTranslationsInner>>,
}

impl Clone for AllTranslations {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
