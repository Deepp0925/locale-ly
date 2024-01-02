use std::collections::HashMap;

use lingual::Lang;

use super::unlocalized::UnlocalizedString;

/// The localized strings contains all translations for the given languages
/// # Fields
/// * `unlocalized` - The unlocalized string
/// * `translated` - A map of translated strings with a key-value pair of the language and the translated string
pub struct LocalizedString<'a> {
    pub unlocalized: &'a UnlocalizedString<'a>,
    pub translated: HashMap<Lang, String>,
}
