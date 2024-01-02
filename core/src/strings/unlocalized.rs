use std::collections::HashMap;

use errors::{Errors, ErrorsResult};
use lingual::{translate, Lang};

use super::LocalizedString;

/// A struct that contains the parsed string and the items that were replaced
/// # Fields
/// * `txt` - The parsed string
/// * `items` - The items that were replaced
/// # Example
/// ```
/// let s = "Hello {name}, there are {count} items in your cart";
/// let parsed = UnlocalizeString::parse_string(s);
/// assert_eq!(parsed.txt, "Hello {0}, there are {1} items in your cart");
/// assert_eq!(parsed.items, vec!["name", "count"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UnlocalizedString<'a> {
    pub txt: &'a str,
    pub items: Vec<String>,
}

impl<'a> UnlocalizedString<'a> {
    /// replaces the interpolated portion of the string with the given items
    /// # Arguments
    /// * `text` - the string to replace the items in
    fn replace(&self, text: &mut String) -> ErrorsResult<()> {
        for (i, item) in self.items.iter().enumerate() {
            let pos = text
                .find(&format!("{{{}}}", i))
                .ok_or_else(|| Errors::FindItemIn(text.to_owned()))?;
            text.replace_range(pos..pos + 3, item);
        }

        Ok(())
    }

    pub async fn translate(
        &self,
        src_lang: Lang,
        target_langs: Vec<Lang>,
    ) -> ErrorsResult<LocalizedString> {
        let mut map = HashMap::with_capacity(target_langs.len());

        for target in target_langs {
            let mut translated = translate(self.txt, src_lang, target).await?.text;
            self.replace(&mut translated)?;
            map.insert(target, translated);
        }

        Ok(LocalizedString {
            unlocalized: self,
            translated: map,
        })
    }
}
