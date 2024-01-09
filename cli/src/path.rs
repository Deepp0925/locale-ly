use errors::{Errors, ErrorsResult};
use lingual::Lang;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt, AsyncWriteExt},
};

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
    fn gen_locale_paths(&self, langs: &[Lang]) -> Vec<PathBuf>;

    async fn create_locale_files(&self, langs: &[Lang]) -> ErrorsResult<HashMap<Lang, File>> {
        let paths = self.gen_locale_paths(langs);
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

    /// removes all the dashes in given files
    /// this particularly happens in yaml which adds: "---" in between mappings
    async fn remove_all_dashes(
        &self,
        langs: &[Lang],
        file_type: ParsedFileType,
    ) -> ErrorsResult<()> {
        if let ParsedFileType::Json = file_type {
            return Ok(());
        }
        let paths = self.gen_locale_paths(langs);
        for path in paths {
            let mut file = TokioFile::open(&path)
                .await
                .map_err(|err| Errors::RemoveDashes(err.to_string()))?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .await
                .map_err(|err| Errors::RemoveDashes(err.to_string()))?;
            let contents = String::from_utf8_lossy(&contents);
            let contents = contents.replace("---\n", "");
            let mut file = TokioFile::create(path)
                .await
                .map_err(|err| Errors::CreateLocaleFile(err.to_string()))?;
            file.write_all(contents.as_bytes())
                .await
                .map_err(|err| Errors::RemoveDashes(err.to_string()))?;
        }
        Ok(())
    }
}

impl<P: AsRef<Path>> LocalePaths for P {
    fn gen_locale_paths(&self, langs: &[Lang]) -> Vec<PathBuf> {
        let path = self.as_ref();
        let mut paths = Vec::with_capacity(langs.len());
        let ext = path
            .extension()
            .expect(&t!("no_file_extension"))
            .to_str()
            .expect(&t!("no_file_extension"));
        for lang in langs {
            let mut path = path.to_path_buf();
            path.set_file_name(format!("{}.{}", lang.as_ref(), ext));
            paths.push(path);
        }
        paths
    }
}
