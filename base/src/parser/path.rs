use errors::{Errors, ErrorsResult};
use lingual::Lang;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::fs::File as TokioFile;

/// Parses relevant information from the path
/// this information pertains to the yml or json
/// and if the language specified
/// This is assuming the format used for the filename is    
/// `en.yml` or `en.json` - `[lang code].[file extension]`

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ParsedFileType {
    #[default]
    Yaml,
    Json,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PathInfo {
    pub lang: Lang,
    pub file_type: ParsedFileType,
}
pub trait ParseInfo: AsRef<Path> {
    fn parse_info(&self) -> Option<PathInfo>;
}

impl<P: AsRef<Path>> ParseInfo for P {
    fn parse_info(&self) -> Option<PathInfo> {
        let path = self.as_ref();
        let file_name = path.file_name()?.to_str()?;
        let mut split = file_name.split('.');
        let lang = split.next()?;
        let file_type = split.next()?;
        let lang = Lang::from_str(lang).ok()?;
        let file_type = match file_type {
            "yml" => ParsedFileType::Yaml,
            "yaml" => ParsedFileType::Yaml,
            "json" => ParsedFileType::Json,
            _ => return None,
        };
        Some(PathInfo { lang, file_type })
    }
}

pub trait LocalePaths {
    fn gen_locale_paths(&self, langs: &[Lang]) -> Option<Vec<PathBuf>>;

    async fn create_locale_files(&self, langs: &[Lang]) -> ErrorsResult<HashMap<Lang, File>> {
        let paths = self
            .gen_locale_paths(langs)
            .ok_or(Errors::CreateLocaleFile(
                "Failed to generate locale paths".to_string(),
            ))?;
        let mut files = HashMap::with_capacity(paths.len());
        let mut options = TokioFile::options();
        let options = options.create(true).read(true).write(true);
        for (i, path) in paths.iter().enumerate() {
            let file = options
                .open(path)
                .await
                .map_err(|err| Errors::CreateLocaleFile(err.to_string()))?
                .into_std()
                .await;

            let file = file;
            files.insert(langs[i], file);
        }

        Ok(files)
    }
}

impl<P: AsRef<Path>> LocalePaths for P {
    fn gen_locale_paths(&self, langs: &[Lang]) -> Option<Vec<PathBuf>> {
        let path = self.as_ref();
        let mut paths = Vec::with_capacity(langs.len());
        let ext = path.extension()?.to_str()?;
        for lang in langs {
            let mut path = path.to_path_buf();
            path.set_file_name(format!("{}.{}", lang.as_ref(), ext));
            paths.push(path);
        }
        Some(paths)
    }
}
