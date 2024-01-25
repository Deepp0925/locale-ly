use errors::{Errors, ErrorsResult};
use lingual::Lang;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::fs::File as TokioFile;

use crate::{
    file_type::FileType,
    serializers::{AllWritersType, SerializerType},
};

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
        let file_type = FileType::<()>::from_str(ext)?;
        Some(PathInfo { lang, file_type })
    }
}

impl<T: AsRef<Path>> LocalePaths for T {}

pub trait LocalePaths: AsRef<Path> {
    /// generate additional paths for src in different formats
    fn gen_src_paths(
        &self,
        file_type: &FileType,
        types: &[FileType],
    ) -> Option<Vec<FileType<PathBuf>>> {
        let path = self.as_ref();
        let mut paths = Vec::with_capacity(types.len() - 1);
        let ext = path.extension()?.to_str()?;
        for ty in types {
            // skip the file type that is already present
            if ty == file_type {
                continue;
            }

            let mut path = path.to_path_buf();
            path.set_extension(ty.as_str(ext));
            paths.push(ty.map(|_| path));
        }
        Some(paths)
    }

    /// create the files for the src in different formats
    /// this is used to write the translated items to
    /// the files
    async fn create_src_files(
        &self,
        file_type: &FileType,
        types: &[FileType],
    ) -> ErrorsResult<Vec<SerializerType>> {
        let paths = self
            .gen_src_paths(file_type, types)
            .ok_or(Errors::CreateFileFormat(
                "Failed to generate src paths".to_string(),
            ))?;

        let mut files = Vec::with_capacity(paths.len());
        let mut options = TokioFile::options();
        let options = options.create(true).read(true).write(true);
        for path in paths {
            let file = options
                .open(path.as_ref())
                .await
                .map_err(|err| Errors::CreateFileFormat(err.to_string()))?
                .into_std()
                .await;
            let serializer = SerializerType::from_file_type(&path, file);
            files.push(serializer);
        }

        Ok(files)
    }

    /// Generates the paths for the locale files
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
                let serializer = SerializerType::from_file_type(&types[i], file);
                serializers.push(serializer);
            }
            files.insert(*lang, serializers);
        }

        Ok(files)
    }
}

impl<T: AsRef<Path>> ParseInfo for T {}
