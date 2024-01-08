use std::borrow::Cow;

use crate::pattern::RegexPattern;

use super::{base::basic_parser, translated::Translated};
use errors::{Errors, ErrorsResult};
use lingual::{translate, Lang};
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
pub struct InterpolatedStr<'a> {
    pub txt: &'a str,
    pub items: Vec<String>,
}

impl<'a> InterpolatedStr<'a> {
    pub fn from_mut_string(s: &'a mut String, regex: Option<RegexPattern>) -> Self {
        basic_parser(s, regex.unwrap_or_default())
    }

    /// replaces the interpolated portion of the string with the given items
    /// # Arguments
    /// * `text` - the string to replace the items in
    fn replace(&self, text: &'a str) -> ErrorsResult<Cow<'a, str>> {
        let mut text = Cow::from(text);

        for (i, item) in self.items.iter().enumerate() {
            let pos = text
                .find(&format!("{{{}}}", i))
                .ok_or_else(|| Errors::FindItemIn(text.to_string()))?;
            text.to_mut().replace_range(pos..pos + 3, item);
        }

        Ok(text)
    }

    pub async fn translate(&self, src_lang: Lang, target_lang: Lang) -> ErrorsResult<String> {
        let translated = translate(self.txt, src_lang, target_lang).await?;
        let translated = self.replace(&translated.text)?;
        println!("{}", translated);

        Ok(translated.into_owned())
    }

    pub async fn translate_bulk(
        &self,
        src_lang: Lang,
        target_lang: &[Lang],
    ) -> ErrorsResult<Vec<Translated>> {
        let mut translated_strs = Vec::with_capacity(target_lang.len());

        for lang in target_lang {
            let translated = self.translate(src_lang, *lang).await?;
            translated_strs.push(Translated {
                txt: translated,
                lang: (*lang),
            });
        }

        Ok(translated_strs)
    }
}

impl From<InterpolatedStr<'_>> for String {
    fn from(s: InterpolatedStr) -> Self {
        s.txt.to_owned()
    }
}
