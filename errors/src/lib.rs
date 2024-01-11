use lingual::TranslationError;
pub type ErrorsResult<T> = Result<T, Errors>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Errors {
    /// No Src Locale provided
    NoSrcLocale,
    /// Unknown File Type resulting in no file being parsed
    UnknownFileType,
    // Translation(TranslationError),
    /// An error occurred while parsing the regex.
    Regex(String),
    /// Unable to find Item in the string.
    /// the error is normally caused when the a string is locating {i} and is unable to find the it
    FindItemIn(String),
    /// Expected String but found something else.
    /// At the very most at the moment only a depth 1 is supported for hashmaps
    /// meaning if the parser locates a Object(hashmap) it must be of type HashMap<String, String>
    /// and not any other type
    ExpectedString(String),
    /// Error Serializing and writing to file
    Serialize(String),
    /// Error creating locale file
    CreateLocaleFile(String),
    /// Error removing dashes
    RemoveDashes(String),
}

impl From<TranslationError> for Errors {
    fn from(_err: TranslationError) -> Self {
        todo!()
    }
}
