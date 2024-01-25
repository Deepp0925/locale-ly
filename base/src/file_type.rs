/// Parses relevant information from the path
/// this information pertains to the yml or json
/// and if the language specified
/// This is assuming the format used for the filename is    
/// `en.yml` or `en.json` - `[lang code].[file extension]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType<T = ()> {
    Yaml(T),
    Json(T),
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Yaml(())
    }
}

impl<T> FileType<T> {
    pub fn from_str(s: &str) -> Option<FileType> {
        match s {
            "yml" => Some(FileType::Yaml(())),
            "yaml" => Some(FileType::Yaml(())),
            "json" => Some(FileType::Json(())),
            _ => None,
        }
    }

    /// returns respective file extension as string
    /// this allows for outputting file in 'yml' or 'yaml' format
    /// based on the source file extension
    pub fn as_str<'a>(&'a self, src_ext: &'a str) -> &str {
        match self {
            FileType::Yaml(_) => src_ext,
            FileType::Json(_) => "json",
        }
    }

    /// maps the return value to a new type
    /// this is useful for converting between types
    /// for example, converting from `FileType<()>` to `FileType<PathBuf>`
    /// or `FileType<PathBuf>` to `FileType<()>`
    /// # Arguments
    /// * `f` - The function to map the value to a new type
    /// # Returns
    /// * `FileType<T>` - The new file type with the mapped value
    pub fn map<F, U>(self, f: F) -> FileType<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            FileType::Yaml(t) => FileType::Yaml(f(t)),
            FileType::Json(t) => FileType::Json(f(t)),
        }
    }
}

impl<T> AsRef<T> for FileType<T> {
    fn as_ref(&self) -> &T {
        match self {
            FileType::Yaml(t) => t,
            FileType::Json(t) => t,
        }
    }
}
