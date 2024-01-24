use errors::{Errors, ErrorsResult};
use lingual::Lang;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::fs::File as TokioFile;

use crate::serializers::{AllWritersType, WriteSerializer};

/// Parses relevant information from the path
/// this information pertains to the yml or json
/// and if the language specified
/// This is assuming the format used for the filename is    
/// `en.yml` or `en.json` - `[lang code].[file extension]`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FileType {
    #[default]
    Yaml,
    Json,
}

impl FileType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "yml" => Some(FileType::Yaml),
            "yaml" => Some(FileType::Yaml),
            "json" => Some(FileType::Json),
            _ => None,
        }
    }

    /// returns respective file extension as string
    /// this allows for outputting file in 'yml' or 'yaml' format
    /// based on the source file extension
    fn as_str<'a>(&'a self, src_ext: &'a str) -> &str {
        match self {
            FileType::Yaml => src_ext,
            FileType::Json => "json",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PathInfo {
    pub lang: Lang,
    pub file_type: FileType,
}
pub trait ParseInfo: AsRef<Path> {
    fn parse_info(&self) -> Option<PathInfo> {
        let path = self.as_ref();
        let file_name = path.file_name()?.to_str()?;
        let mut split = file_name.split('.');
        let lang = split.next()?;
        let ext = split.next()?;
        let lang = Lang::from_str(lang).ok()?;
        let file_type = FileType::from_str(ext)?;
        Some(PathInfo { lang, file_type })
    }
}

impl<T: AsRef<Path>> LocalePaths for T {}

pub trait LocalePaths: AsRef<Path> {
    fn gen_locale_paths(
        &self,
        langs: &[Lang],
        types: &[FileType],
    ) -> Option<HashMap<Lang, Vec<PathBuf>>> {
        let path = self.as_ref();
        let mut paths = HashMap::with_capacity(langs.len());
        let ext = path.extension()?.to_str()?;
        for lang in langs {
            let mut lang_paths = Vec::with_capacity(types.len());
            for ty in types {
                let mut path = path.to_path_buf();
                path.set_file_name(format!("{}.{}", lang.as_ref(), ty.as_str(ext)));
                lang_paths.push(path);
            }
            paths.insert(*lang, lang_paths);
        }
        Some(paths)
    }

    async fn create_locale_files(
        &self,
        langs: &[Lang],
        types: &[FileType],
    ) -> ErrorsResult<AllWritersType> {
        let paths_collection =
            self.gen_locale_paths(langs, types)
                .ok_or(Errors::CreateLocaleFile(
                    "Failed to generate locale paths".to_string(),
                ))?;
        let mut files = HashMap::with_capacity(paths_collection.len());
        let mut options = TokioFile::options();
        let options = options.create(true).read(true).write(true);
        for (lang, paths) in paths_collection.iter() {
            let mut serializers = Vec::with_capacity(paths.len());
            for (i, path) in paths.iter().enumerate() {
                let file = options
                    .open(path)
                    .await
                    .map_err(|err| Errors::CreateLocaleFile(err.to_string()))?
                    .into_std()
                    .await;
                let serializer = WriteSerializer::from_file_type(&types[i], file);
                serializers.push(serializer);
            }
            files.insert(*lang, serializers);
        }

        Ok(files)
    }
}

impl<T: AsRef<Path>> ParseInfo for T {}
