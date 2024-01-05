use errors::{Errors, ErrorsResult};
use regex::Regex;

/// The default regex pattern will match all items
/// that are wrapped in curly braces, i.e. `{item}`
pub const DEFAULT_REGEX_PATTERN: &str = r"\{(\w+)\}";

/// ruby uses `%{item}` to interpolate strings, therefore
/// this regex pattern will match all items that are wrapped
/// in `%{item}`
pub const RUBY_REGEX_PATTERN: &str = r"%\{(\w+)\}";

#[derive(Debug, Clone, Default)]
pub enum RegexPattern {
    /// The default regex pattern will match all items
    /// that are wrapped in curly braces, i.e. `{item}`
    #[default]
    Default,
    /// ruby uses `%{item}` to interpolate strings, therefore
    /// this regex pattern will match all items that are wrapped
    /// in `%{item}`
    Ruby,
    /// A custom regex pattern
    Custom(String),
}

impl Default for &RegexPattern {
    fn default() -> Self {
        &RegexPattern::Default
    }
}

impl From<RegexPattern> for Regex {
    fn from(pattern: RegexPattern) -> Self {
        pattern.regex().unwrap()
    }
}

impl RegexPattern {
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
