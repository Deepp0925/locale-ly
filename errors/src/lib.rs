use lingual::Errors as TranslationError;
pub type ErrorsResult<T> = Result<T, Errors>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Errors {
    // Translation(TranslationError),
    /// An error occurred while parsing the regex.
    Regex(String),
    /// Unable to find Item in the string.
    /// the error is normally caused when the a string is locating {i} it is unable to find the it
    FindItemIn(String),
}

impl From<TranslationError> for Errors {
    fn from(_err: TranslationError) -> Self {
        todo!()
    }
}
