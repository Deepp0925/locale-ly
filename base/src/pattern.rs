use errors::{Errors, ErrorsResult};
use regex::Regex;

/// The default regex pattern will match all items
/// that are wrapped in curly braces, i.e. `{item}`
pub const DEFAULT_REGEX_PATTERN: &str = r"\{(\w+)\}";

/// ruby uses `%{item}` to interpolate strings, therefore
/// this regex pattern will match all items that are wrapped
/// in `%{item}`
pub const RUBY_REGEX_PATTERN: &str = r"%\{(\w+)\}";

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum RegexPattern<'a> {
    /// The default regex pattern will match all items
    /// that are wrapped in curly braces, i.e. `{item}`
    #[default]
    Default,
    /// ruby uses `%{item}` to interpolate strings, therefore
    /// this regex pattern will match all items that are wrapped
    /// in `%{item}`
    Ruby,
    /// A custom regex pattern
    Custom(&'a str),
}

impl Default for &RegexPattern<'_> {
    fn default() -> Self {
        &RegexPattern::Default
    }
}

impl From<RegexPattern<'_>> for Regex {
    fn from(pattern: RegexPattern) -> Self {
        pattern.regex().unwrap()
    }
}

impl From<&RegexPattern<'_>> for Regex {
    fn from(pattern: &RegexPattern) -> Self {
        pattern.regex().unwrap()
    }
}

impl<'a> RegexPattern<'a> {
    pub fn from_str(s: &str, p: Option<&'a str>) -> Option<RegexPattern<'a>> {
        let is_not_custom = match s {
            "default" => Some(RegexPattern::Default),
            "ruby" => Some(RegexPattern::Ruby),
            _ => None,
        };

        if let Some(pattern) = is_not_custom {
            return Some(pattern);
        }

        p.map(RegexPattern::Custom)
    }

    pub fn regex(&self) -> ErrorsResult<Regex> {
        match self {
            // safe to unwrap the following two as they are hardcoded and
            // will always be valid regex patterns
            RegexPattern::Default => Ok(Regex::new(DEFAULT_REGEX_PATTERN).unwrap()),
            RegexPattern::Ruby => Ok(Regex::new(RUBY_REGEX_PATTERN).unwrap()),
            // // Custom regex patterns are not hardcoded and therefore
            // // can be invalid and return an error if they fail
            RegexPattern::Custom(pattern) => {
                Regex::new(pattern).map_err(|err| Errors::Regex(err.to_string()))
            }
        }
    }
}
