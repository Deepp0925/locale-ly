use std::borrow::Cow;

use crate::pattern::RegexPattern;

use super::base::basic_parser;
use errors::{Errors, ErrorsResult};
use lingual::{Lang, Translator};
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
pub struct Interpolated {
    pub txt: String,
    pub items: Vec<String>,
}

impl Interpolated {
    pub fn from_string(s: String, regex: Option<RegexPattern>) -> Self {
        basic_parser(s, regex.unwrap_or_default())
    }

    /// replaces the interpolated portion of the string with the given items
    /// # Arguments
    /// * `text` - the string to replace the items in
    fn replace<'a>(&self, text: &'a str) -> ErrorsResult<Cow<'a, str>> {
        let mut text = Cow::from(text);

        for (i, item) in self.items.iter().enumerate() {
            let start = text
                .find(&format!("{{{}}}", i))
                .ok_or_else(|| Errors::FindItemIn(text.to_string()))?;

            let end = if i > 9 {
                // 10th item and later, the index will be 4 characters long { + num + } = 4
                start + 4
            } else {
                start + 3
            };
            text.to_mut().replace_range(start..end, item);
        }

        Ok(text)
    }

    pub async fn translate(
        &self,
        translator: &Translator,
        src_lang: &Lang,
        target_lang: &Lang,
    ) -> ErrorsResult<String> {
        let translated = translator
            .translate(&self.txt, src_lang, target_lang)
            .await?;
        let translated = self.replace(&translated.text)?;

        Ok(translated.escape_unicode().to_string())
    }
}

#[test]
fn test_interpolated_str() {
    let s = "Hello {name}, there are {count} items in your cart".to_string();
    let parsed = Interpolated::from_string(s, None);
    assert_eq!(parsed.txt, "Hello {0}, there are {1} items in your cart");
    assert_eq!(parsed.items, vec!["{name}", "{count}"]);

    let  s = "Hello {name}, {there} are {count} {items} in your {cart}. Please {check} page {for} more {details}. Also {other} contains {notification}. Additional info @{location} available at {here}".to_string();
    let s_cloned = s.clone();
    let parsed = Interpolated::from_string(s, None);
    assert_eq!(parsed.txt, "Hello {0}, {1} are {2} {3} in your {4}. Please {5} page {6} more {7}. Also {8} contains {9}. Additional info @{10} available at {11}");
    assert_eq!(
        parsed.items,
        vec![
            "{name}",
            "{there}",
            "{count}",
            "{items}",
            "{cart}",
            "{check}",
            "{for}",
            "{details}",
            "{other}",
            "{notification}",
            "{location}",
            "{here}"
        ]
    );

    assert_eq!(s_cloned, parsed.replace(&parsed.txt).unwrap());
}
