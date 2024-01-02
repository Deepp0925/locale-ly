use super::base::basic_parser;
use errors::{Errors, ErrorsResult};
use lingual::{translate, AccurateLang};
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
        src_lang: AccurateLang,
        target_lang: AccurateLang,
    ) -> ErrorsResult<String> {
        let mut translated = translate(self.txt, src_lang, target_lang).await?.text;
        self.replace(&mut translated)?;

        Ok(translated)
    }
}

impl From<InterpolatedStr<'_>> for String {
    fn from(s: InterpolatedStr) -> Self {
        s.txt.to_owned()
    }
}

impl<'a> From<&'a mut String> for InterpolatedStr<'a> {
    fn from(s: &'a mut String) -> Self {
        basic_parser(s, Default::default())
    }
}
