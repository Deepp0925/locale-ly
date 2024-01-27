use lingual::TranslationError;
pub type ErrorsResult<T> = Result<T, Errors>;

/// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("../assets/locales", fallback = "en");

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Errors {
    /// No Src Locale provided
    NoSrcLocale,
    /// Unknown File Type resulting in no file being parsed
    UnknownFileType,
    /// There was an error opening the file
    OpenFile(String),
    /// There was an error parsing the json
    InvalidJson(String),
    /// There was an error parsing the yaml
    InvalidYaml(String),
    // Translation(TranslationError),
    /// An error occurred while parsing the regex.
    Regex(String),
    /// Unable to find Item in the string.
    /// the error is normally caused when the a string is locating {i} and is unable to find the it
    FindItemIn(String),
    /// Expected String, Array or Object
    UnexpectedItem,
    /// Error Serializing and writing to file
    Serialize(String),
    /// Error creating locale file
    CreateLocaleFile(String),
    /// Error creating file format
    CreateFileFormat(String),
    /// An error occurred while translating the string
    Translation(Cow<'static, str>),
}

impl Errors {
    pub fn to_str(self) -> Cow<'static, str> {
        match self {
            Errors::NoSrcLocale => t!("no_src_locale_err"),
            Errors::UnknownFileType => t!("unknown_file_type_err"),
            Errors::OpenFile(err) => t!("open_file_err", err = err),
            Errors::InvalidJson(err) => t!("invalid_json_err", err = err),
            Errors::InvalidYaml(err) => t!("invalid_yaml_err", err = err),
            Errors::Regex(err) => t!("regex_err", err = err),
            Errors::FindItemIn(translated_txt) => t!("find_item_in_err", string = translated_txt),
            Errors::UnexpectedItem => t!("unexpected_item_err"),
            Errors::Serialize(err) => t!("serialize_err", err = err),
            Errors::CreateLocaleFile(err) => t!("create_locale_file_err", err = err),
            Errors::CreateFileFormat(err) => t!("create_file_format_err", err = err),
            Errors::Translation(err) => err,
        }
    }
}

impl From<TranslationError> for Errors {
    fn from(err: TranslationError) -> Self {
        match err {
            TranslationError::ParseIntErr(string) => {
                Errors::Translation(t!("parse_int_err", string = string))
            }
            TranslationError::HttpErr(s) => Errors::Translation(t!("http_err", err = s)),
            TranslationError::UrlParseErr(string) => {
                Errors::Translation(t!("url_parse_err", string = string))
            }
            TranslationError::JsonParseErr(s) => Errors::InvalidJson(s),
        }
    }
}
